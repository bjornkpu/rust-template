#!/usr/bin/env bash
# Stop hook: the gate. Claude cannot end a turn while fmt, clippy or nextest is red.
#
# Skips when no Rust source changed since the last green run. When the gate is red and the tree
# is unchanged since the last block, Claude could not fix it: let the turn end and tell the user,
# instead of looping.
set -u
cd "${CLAUDE_PROJECT_DIR:-.}" || exit 0

state=target/.gate
mkdir -p "$state"
hash=$(git ls-files -co --exclude-standard -- '*.rs' 'Cargo.toml' 'Cargo.lock' 'tests/*' \
  | sort -u | tr '\n' '\0' | xargs -0 -r sha1sum 2>/dev/null | sha1sum | cut -d' ' -f1)

if [ "$hash" = "$(cat "$state/green" 2>/dev/null)" ]; then
  rm -f "$state/red"
  exit 0
fi

out=$( { cargo fmt --check && cargo clippy --all-targets --all-features --quiet \
  && cargo nextest run --all-features --no-tests=pass; } 2>&1 )
status=$?

if [ "$status" -eq 0 ]; then
  echo "$hash" > "$state/green"
  rm -f "$state/red"
  exit 0
fi

tail=$(printf '%s\n' "$out" | tail -n 40)
if [ "$hash" = "$(cat "$state/red" 2>/dev/null)" ]; then
  printf 'GATE RED, unchanged since the last block. Ending the turn so BK can look:\n%s\n' "$tail" >&2
  exit 1
fi
echo "$hash" > "$state/red"
printf 'Gate red (fmt, clippy, nextest). Fix it before finishing:\n%s\n' "$tail" >&2
exit 2
