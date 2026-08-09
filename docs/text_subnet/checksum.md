# `checksum` — Hash Verification

Computes MD5, SHA-256, or SHA-512 checksums of a file or stdin.

## Usage

```
cmdutils text checksum [--algo <md5|sha256|sha512>] [input]
```

| Argument | Description |
|----------|-------------|
| `--algo` | Hash algorithm (default `sha256`) |
| `input`  | File to hash (defaults to stdin) |

## Examples

```
cmdutils text checksum file.iso
cmdutils text checksum file.iso --algo sha512
cmdutils text checksum archive.tar.gz --algo md5
echo -n "hello" | cmdutils text checksum
```

## Output

```
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  sha256  file.txt
```

## Errors

| Condition | Behaviour |
|-----------|-----------|
| Unknown algorithm | Error: _Unsupported algorithm 'x'. Valid: md5, sha256, sha512_ |

> **Note:** MD5 is cryptographically broken and should only be used for
> legacy checksum verification — prefer SHA-256.
