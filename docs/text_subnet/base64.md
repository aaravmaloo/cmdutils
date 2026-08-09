# `base64` — Base64 Encoding

Encodes or decodes base64 data from a file or stdin.

## Usage

```
cmdutils text base64 encode [options] [input]
cmdutils text base64 decode [options] [input]
```

| Argument   | Description |
|------------|-------------|
| `input`    | File to read (defaults to stdin) |
| `-o <path>`| Write output to a file (instead of stdout) |

## Examples

```
cmdutils text base64 encode photo.png -o photo.txt     # base64 of a file
cmdutils text base64 decode photo.txt -o photo.png     # back to bytes
echo -n "hello world" | cmdutils text base64 encode    # aGVsbG8gd29ybGQ=
```

Standard (RFC 4648) base64 with padding is used. Decode trims surrounding
whitespace.

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Non-base64 input (decode) | Error: _Invalid base64 input_ |
| Non-UTF-8 input (decode) | Error: _Input is not valid UTF-8 base64 text_ |
