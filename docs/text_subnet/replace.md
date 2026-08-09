# `replace` — Find & Replace

Replaces all occurrences of a string in a file or stdin.

## Usage

```
cmdutils text replace <find> <replace> [options] [input]
```

| Argument   | Description |
|------------|-------------|
| `find`     | Text to search for |
| `replace`  | Replacement text |
| `input`    | File to read (defaults to stdin) |
| `-i`       | Rewrite the input file in place |
| `-o <path>`| Write the result to a file (instead of stdout) |

## Examples

```
# Print result to stdout
cmdutils text replace "TODO" "DONE" notes.txt

# Modify the file in place
cmdutils text replace "http://" "https://" links.txt -i

# Write to a new file
cmdutils text replace "foo" "bar" in.txt -o out.txt

# Pipe-friendly
cat file.txt | cmdutils text replace "a" "b"
```

Replacement is literal — no regex support.

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Empty `find` | Error: _Search string cannot be empty_ |
| `-i` with `-o` | Error: _Cannot use both --in-place and --output_ |
| `-i` without a file | Error: _--in-place requires an input file_ |
