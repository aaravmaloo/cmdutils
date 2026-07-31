#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: ./update.sh <version>"
    echo "Example: ./update.sh v0.0.2"
    exit 1
fi

VERSION="${1#v}"  # strip leading 'v' if present
TAG="v$VERSION"

# sha256sum (GNU/Linux, incl. Arch CI container) vs shasum (macOS local use)
if command -v sha256sum >/dev/null 2>&1; then
    sha256() { sha256sum "$1" | awk '{print $1}'; }
else
    sha256() { shasum -a 256 "$1" | awk '{print $1}'; }
fi

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "==> Downloading release tarballs for $TAG..."
curl -fsSL "https://github.com/aaravmaloo/cmdutils/releases/download/$TAG/cmdutils-${TAG}-linux_x64.tar.gz" -o "$TMPDIR/x64.tar.gz"
curl -fsSL "https://github.com/aaravmaloo/cmdutils/releases/download/$TAG/cmdutils-${TAG}-linux_arm64.tar.gz" -o "$TMPDIR/arm64.tar.gz"

AMD64=$(sha256 "$TMPDIR/x64.tar.gz")
ARM64=$(sha256 "$TMPDIR/arm64.tar.gz")

if [ -z "$AMD64" ] || [ -z "$ARM64" ]; then
    echo "ERROR: Could not compute hashes. Does release $TAG exist?"
    exit 1
fi

echo "==> amd64: $AMD64"
echo "==> arm64: $ARM64"

echo "==> Updating PKGBUILD..."

# NOTE: sed -i.bak + rm works on BOTH GNU sed (Linux/Arch container)
# and BSD sed (macOS), so this script is safe locally and in CI.
sed -i.bak \
    -e "s/^pkgver=.*/pkgver=$VERSION/" \
    -e "s/^sha256sums_x86_64=('.*')/sha256sums_x86_64=('$AMD64')/" \
    -e "s/^sha256sums_aarch64=('.*')/sha256sums_aarch64=('$ARM64')/" \
    PKGBUILD
rm -f PKGBUILD.bak

echo "==> Regenerating .SRCINFO..."
makepkg --printsrcinfo > .SRCINFO

echo "==> Committing and pushing..."
git add PKGBUILD .SRCINFO
if git diff --cached --quiet; then
    echo "No AUR changes to publish."
    exit 0
fi

git commit -m "update to v$VERSION"
git push

echo "==> Done! cmdutils-bin $VERSION is live on AUR."
