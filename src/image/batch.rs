use std::path::{Path, PathBuf};

/// Expand a CLI input into a list of files.
///
/// Supports glob patterns (`*.png`, `photos/*.jpg`) and plain file paths.
pub fn expand_inputs(input: &str) -> Result<Vec<PathBuf>, String> {
    let has_glob = input.contains(['*', '?', '[', '{']);

    if has_glob {
        let mut paths: Vec<PathBuf> = glob::glob(input)
            .map_err(|e| format!("Invalid glob pattern '{input}': {e}"))?
            .filter_map(|r| r.ok())
            .collect();
        paths.sort();
        if paths.is_empty() {
            return Err(format!("No files matched pattern '{input}'"));
        }
        return Ok(paths);
    }

    if Path::new(input).exists() {
        return Ok(vec![PathBuf::from(input)]);
    }

    Err(format!("Input file not found: {input}"))
}

/// Run a worker over one or more inputs.
///
/// A single input is processed directly; multiple inputs (from a glob) are
/// processed in parallel with bounded concurrency, and failures are reported
/// per-file at the end.
pub fn run_batch<F>(input: &str, worker: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&str) -> Result<(), Box<dyn std::error::Error>> + Sync + Send,
{
    let inputs = expand_inputs(input)?;
    let worker = &worker;

    if inputs.len() == 1 {
        let path = inputs[0].to_str().ok_or("Input path is not valid UTF-8")?;
        return worker(path);
    }

    let parallelism = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(inputs.len());

    let mut results: Vec<(String, Result<(), String>)> = Vec::with_capacity(inputs.len());
    for chunk in inputs.chunks(parallelism) {
        std::thread::scope(|scope| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|p| {
                    let path = p.to_string_lossy().to_string();
                    scope.spawn(move || {
                        let res = worker(&path).map_err(|e| e.to_string());
                        (path, res)
                    })
                })
                .collect();
            for handle in handles {
                let (path, res) = handle
                    .join()
                    .map_err(|_| "worker thread panicked".to_string())
                    .unwrap();
                results.push((path, res));
            }
        });
    }

    let total = results.len();
    let ok = results.iter().filter(|(_, r)| r.is_ok()).count();
    let failed = total - ok;

    if failed > 0 {
        eprintln!("Processed {ok}/{total} files, {failed} failed:");
        for (path, res) in &results {
            if let Err(e) = res {
                eprintln!("  ✗ {path}: {e}");
            }
        }
        return Err(format!("{failed} of {total} files failed").into());
    }

    println!("✅ Processed {total} files");
    Ok(())
}
