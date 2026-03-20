#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ORIGIN_URL="${CODEX_ORIGIN_URL:-https://github.com/zapabob/codex.git}"
UPSTREAM_URL="${CODEX_UPSTREAM_URL:-https://github.com/openai/codex.git}"
TARGET_BRANCH="${CODEX_TARGET_BRANCH:-main}"
UPSTREAM_BRANCH="${CODEX_UPSTREAM_BRANCH:-main}"
METADATA_PATH="${CODEX_UPSTREAM_METADATA:-releases/upstream-sync.json}"
UPSTREAM_TAG_NAMESPACE="refs/upstream-tags"
UPSTREAM_RELEASE_TAG_PATTERN="rust-v*"
UPSTREAM_LEGACY_TAG_PATTERN="v*"
CONFIGURE_ONLY=0
DRY_RUN=0
SKIP_FETCH=0
SKIP_MERGE=0

usage() {
  cat <<USAGE
Usage: scripts/sync-upstream.sh [options]

Options:
  --configure-only   Define canonical origin/upstream remotes and tracking refspecs only.
  --dry-run          Fetch and record metadata but do not merge.
  --skip-fetch       Reuse existing refs without fetching.
  --skip-merge       Do not run git merge (implies metadata refresh only).
  -h, --help         Show this help.

Environment overrides:
  CODEX_ORIGIN_URL, CODEX_UPSTREAM_URL, CODEX_TARGET_BRANCH,
  CODEX_UPSTREAM_BRANCH, CODEX_UPSTREAM_METADATA.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --configure-only)
      CONFIGURE_ONLY=1
      ;;
    --dry-run)
      DRY_RUN=1
      ;;
    --skip-fetch)
      SKIP_FETCH=1
      ;;
    --skip-merge)
      SKIP_MERGE=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
  shift
done

if [[ ! -d .git ]]; then
  echo "error: $ROOT_DIR is not a git repository" >&2
  exit 1
fi

configure_remote() {
  local name="$1"
  local url="$2"
  shift 2

  if git remote get-url "$name" >/dev/null 2>&1; then
    git remote set-url "$name" "$url"
  else
    git remote add "$name" "$url"
  fi

  git config --unset-all "remote.${name}.fetch" >/dev/null 2>&1 || true
  for refspec in "$@"; do
    git config --add "remote.${name}.fetch" "$refspec"
  done
}

configure_remote \
  origin \
  "$ORIGIN_URL" \
  "+refs/heads/*:refs/remotes/origin/*" \
  "+refs/tags/*:refs/tags/*"

configure_remote \
  upstream \
  "$UPSTREAM_URL" \
  "+refs/heads/${UPSTREAM_BRANCH}:refs/remotes/upstream/${UPSTREAM_BRANCH}" \
  "+refs/tags/${UPSTREAM_RELEASE_TAG_PATTERN}:${UPSTREAM_TAG_NAMESPACE}/${UPSTREAM_RELEASE_TAG_PATTERN}" \
  "+refs/tags/${UPSTREAM_LEGACY_TAG_PATTERN}:${UPSTREAM_TAG_NAMESPACE}/${UPSTREAM_LEGACY_TAG_PATTERN}"

git config branch."$TARGET_BRANCH".remote origin
git config branch."$TARGET_BRANCH".merge refs/heads/"$TARGET_BRANCH"

echo "Configured remotes:"
git remote -v

echo
echo "Configured fetch refspecs:"
git config --get-all remote.origin.fetch
git config --get-all remote.upstream.fetch

if [[ "$CONFIGURE_ONLY" -eq 1 ]]; then
  exit 0
fi

if [[ "$SKIP_FETCH" -eq 0 ]]; then
  git fetch origin --prune --quiet
  git fetch upstream --prune --quiet
fi

upstream_ref="refs/remotes/upstream/${UPSTREAM_BRANCH}"
if ! git rev-parse --verify "$upstream_ref" >/dev/null 2>&1; then
  echo "error: missing ${upstream_ref}; run without --skip-fetch or configure upstream first" >&2
  exit 1
fi

upstream_commit="$(git rev-parse "$upstream_ref")"
upstream_short="$(git rev-parse --short=12 "$upstream_ref")"
exact_upstream_tag="$(git for-each-ref --format='%(refname:strip=2)' --points-at="$upstream_ref" "$UPSTREAM_TAG_NAMESPACE" | sort -V | tail -n 1 || true)"
if [[ -z "$exact_upstream_tag" ]]; then
  exact_upstream_tag="null"
fi

python - "$METADATA_PATH" "$ORIGIN_URL" "$UPSTREAM_URL" "$TARGET_BRANCH" "$UPSTREAM_BRANCH" "$upstream_commit" "$exact_upstream_tag" <<'PY'
import json
import pathlib
import sys
from datetime import datetime, timezone

path = pathlib.Path(sys.argv[1])
origin_url, upstream_url, target_branch, upstream_branch, upstream_commit, exact_tag = sys.argv[2:8]
recorded_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace('+00:00', 'Z')
source_tag = None if exact_tag == 'null' else exact_tag

data = {
    "schema_version": 1,
    "remotes": {
        "origin": {
            "name": "origin",
            "url": origin_url,
            "fetch": [
                "+refs/heads/*:refs/remotes/origin/*",
                "+refs/tags/*:refs/tags/*",
            ],
        },
        "upstream": {
            "name": "upstream",
            "url": upstream_url,
            "tracked_branch": upstream_branch,
            "fetch": [
                f"+refs/heads/{upstream_branch}:refs/remotes/upstream/{upstream_branch}",
                "+refs/tags/rust-v*:refs/upstream-tags/rust-v*",
                "+refs/tags/v*:refs/upstream-tags/v*",
            ],
            "release_tag_policy": {
                "primary_pattern": "rust-v*",
                "secondary_pattern": "v*",
                "exact_tag_required_for_source_tag": True,
            },
        },
    },
    "sync": {
        "target_branch": target_branch,
        "merge_strategy": "git merge --no-ff upstream/main",
        "recorded_at": recorded_at,
        "source": {
            "repository": upstream_url,
            "branch": upstream_branch,
            "commit": upstream_commit,
            "tag": source_tag,
        },
        "conflict_policy_document": "docs/repository-relationship.md",
    },
}
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
PY

echo
echo "Recorded upstream metadata to ${METADATA_PATH}:"
cat "$METADATA_PATH"

if [[ "$DRY_RUN" -eq 1 || "$SKIP_MERGE" -eq 1 ]]; then
  echo
  echo "Skipped merge; upstream/main currently resolves to ${upstream_short}."
  exit 0
fi

current_branch="$(git branch --show-current)"
if [[ "$current_branch" != "$TARGET_BRANCH" ]]; then
  echo "error: current branch is ${current_branch}; switch to ${TARGET_BRANCH} before merging" >&2
  exit 1
fi

if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "error: working tree is dirty; commit or stash changes before merge" >&2
  exit 1
fi

git merge --no-ff "$upstream_ref" -m "Merge upstream/${UPSTREAM_BRANCH}: import ${upstream_short}"

echo
echo "Merge complete from upstream/${UPSTREAM_BRANCH} (${upstream_short})."
