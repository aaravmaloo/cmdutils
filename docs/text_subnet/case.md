# `case` — Letter-Case Conversion

Converts text between common letter-case styles.

## Usage

```
cmdutils text case --to <style> [--text <text>] [input]
```

| Argument | Description |
|----------|-------------|
| `--to`   | Target style: `upper`, `lower`, `title`, `snake`, `kebab`, `camel`, `pascal`, `constant` |
| `--text` | Literal text to convert (instead of reading a file) |
| `input`  | File to read (defaults to stdin) |

## Examples

```
cmdutils text case --to snake --text "Hello World"     →  hello_world
cmdutils text case --to kebab --text "UserProfile"     →  user-profile
cmdutils text case --to pascal --text "hello world"    →  HelloWorld
cmdutils text case --to constant --text "foo bar"      →  FOO_BAR
cmdutils text case --to title file.txt
```

Word-based styles split on non-alphanumeric characters and also split
camelCase / PascalCase transitions (`UserProfile` → `user`, `profile`).

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Unknown style | Error: _Unknown case 'x'. Valid: ..._ |
