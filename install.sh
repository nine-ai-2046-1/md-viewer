#!/usr/bin/env bash
# md-viewer installer — macOS, no sudo, no admin rights required.
#
#   ./install.sh                      install latest into ~/.local/bin
#   ./install.sh --prefix ~/bin       pick another directory
#   ./install.sh --version 0.1.0      pin a specific version
#   ./install.sh --uninstall          remove the binary
#
# Env vars: PREFIX, VERSION, GITHUB_TOKEN (token only needed for private repos)
set -euo pipefail

NAME="md-viewer"
REPO="nine-ai-2046-1/md-viewer"

PREFIX="${PREFIX:-$HOME/.local/bin}"
VERSION="${VERSION:-}"
TOKEN="${GITHUB_TOKEN:-}"
UNINSTALL=0

step() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
ok()   { printf '\033[1;32m  OK\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m  !!\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31m  XX\033[0m %s\n' "$*" >&2; exit 1; }

fetch_api() {
  if [ -n "$TOKEN" ]; then
    curl -fsSL -H "Authorization: Bearer $TOKEN" -H "Accept: application/vnd.github+json" "$1"
  else
    curl -fsSL -H "Accept: application/vnd.github+json" "$1"
  fi
}

fetch_file() {
  if [ -n "$TOKEN" ]; then
    curl -fL --retry 3 -o "$2" -H "Authorization: Bearer $TOKEN" "$1"
  else
    curl -fL --retry 3 -o "$2" "$1"
  fi
}

while [ $# -gt 0 ]; do
  case "$1" in
    --prefix)    PREFIX="$2"; shift 2 ;;
    --prefix=*)  PREFIX="${1#*=}"; shift ;;
    --version)   VERSION="$2"; shift 2 ;;
    --version=*) VERSION="${1#*=}"; shift ;;
    --uninstall) UNINSTALL=1; shift ;;
    -h|--help)   sed -n '2,10p' "$0" | sed 's/^# */  /'; exit 0 ;;
    *)           die "Unknown option: $1" ;;
  esac
done

if [ "$UNINSTALL" = "1" ]; then
  if [ -f "$PREFIX/$NAME" ]; then
    rm -f "$PREFIX/$NAME"
    ok "Removed $PREFIX/$NAME"
  else
    warn "$PREFIX/$NAME does not exist"
  fi
  echo "If '$NAME' is still resolvable, delete the PATH line this installer added to your shell rc file."
  exit 0
fi

[ "$(uname -s)" = "Darwin" ] || die "This installer only handles macOS. Elsewhere: cargo build --release --locked"

case "$(uname -m)" in
  arm64)  TARGET="aarch64-apple-darwin" ;;
  x86_64)
    TARGET="x86_64-apple-darwin"
    # An x86 shell on Apple Silicon (Rosetta) still wants the arm64 build.
    if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null || echo 0)" = "1" ]; then
      TARGET="aarch64-apple-darwin"
    fi ;;
  *)      die "Unsupported CPU architecture: $(uname -m)" ;;
esac
step "Detected macOS $(uname -m) -> $TARGET"

if [ -z "$VERSION" ]; then
  step "Resolving the latest release of $REPO"
  TAG=$(fetch_api "https://api.github.com/repos/$REPO/releases/latest" | grep -m1 '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/' || true)
  [ -n "${TAG:-}" ] || die "Could not resolve the latest release. Private repo? Set GITHUB_TOKEN=... and retry."
  VERSION="${TAG#v}"
fi

FILE="$NAME-$VERSION-$TARGET"
URL="https://github.com/$REPO/releases/download/v$VERSION/$FILE.tar.gz"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/$NAME.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

step "Downloading $FILE.tar.gz"
fetch_file "$URL" "$TMP/$FILE.tar.gz" || die "Download failed: $URL"

if fetch_file "$URL.sha256" "$TMP/$FILE.tar.gz.sha256" 2>/dev/null; then
  if ( cd "$TMP" && shasum -a 256 -c "$FILE.tar.gz.sha256" >/dev/null 2>&1 ); then
    ok "sha256 verified"
  else
    die "Checksum mismatch - the download looks corrupted. Run the script again."
  fi
else
  warn "No .sha256 sidecar found, skipping checksum verification"
fi

step "Extracting"
tar -xzf "$TMP/$FILE.tar.gz" -C "$TMP"
[ -f "$TMP/$FILE/$NAME" ] || die "Archive did not contain '$NAME'"

step "Installing into $PREFIX"
[ -d "$PREFIX" ] || mkdir -p "$PREFIX" || die "Cannot create $PREFIX"
[ -w "$PREFIX" ] || die "$PREFIX is not writable by $(whoami). Try: ./install.sh --prefix ~/bin"
cp "$TMP/$FILE/$NAME" "$PREFIX/$NAME"
chmod 755 "$PREFIX/$NAME"
# Belt-and-braces: drop the quarantine flag so Gatekeeper never prompts.
xattr -d com.apple.quarantine "$PREFIX/$NAME" 2>/dev/null || true
ok "$PREFIX/$NAME  (v$VERSION, $TARGET)"

add_path() {
  rc="$1"
  [ -f "$rc" ] || { mkdir -p "$(dirname "$rc")"; : > "$rc"; }
  if grep -qF "$PREFIX" "$rc" 2>/dev/null; then
    ok "$rc already references $PREFIX"
    return 0
  fi
  {
    echo ""
    echo "# $NAME installer - user-local bin (no admin rights needed)"
    echo "export PATH=\"$PREFIX:\$PATH\""
  } >> "$rc"
  ok "Added $PREFIX to $rc"
}

case "$(basename "${SHELL:-/bin/zsh}")" in
  zsh)  RC="$HOME/.zshrc" ;;
  bash) RC="$HOME/.bash_profile" ;;
  *)    RC="$HOME/.profile" ;;
esac
add_path "$RC"

if "$PREFIX/$NAME" --help >/dev/null 2>&1 || "$PREFIX/$NAME" --version >/dev/null 2>&1; then
  ok "Binary runs"
else
  warn "Installed, but '$NAME --help' did not exit cleanly - check it manually"
fi

OTHER=$(command -v "$NAME" 2>/dev/null || true)
if [ -n "$OTHER" ] && [ "$OTHER" != "$PREFIX/$NAME" ]; then
  warn "Another copy exists at $OTHER and may shadow this one"
fi

cat <<EOF

Done - no sudo was used, everything lives under your home directory.

  binary : $PREFIX/$NAME
  version: v$VERSION ($TARGET)
  shell  : $RC

Open a new terminal (or run: source $RC), then:

  $NAME --help

Uninstall:
  $PREFIX/install.sh --uninstall   # or just: rm $PREFIX/$NAME
EOF
