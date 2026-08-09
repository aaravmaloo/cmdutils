# `count` — Text Counting

Counts lines, words, characters, and bytes of a file or stdin (wc-style).

## Usage

```
cmdutils text count [options] [input]
```

| Argument  | Description |
|-----------|-------------|
| `input`   | File to count. Omit or use `-` to read from stdin. |
| `-w`      | Words only |
| `-l`      | Lines only |
| `-m`      | Characters only |
| `-c`      | Bytes only |

With no flags, all four counts are printed.

## Behaviour

- **Lines** counts newline (`\n`) terminators (same as `wc -l`).
- **Words** splits on Unicode whitespace.
- **Characters** counts Unicode code points.
- **Bytes** counts raw file size.

## Output

```
      2 lines,       5 words,      24 chars,      24 bytes  sample.txt
```

Piped usage:

```
cat file.txt | cmdutils text count
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Missing file | Error: _No such file or directory_ |
