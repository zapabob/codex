#!/bin/bash
set -e

echo "🔄 Syncing with upstream openai/codex..."
echo "Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
echo "User: zapabob"
echo ""

# Add upstream remote if not exists
if ! git remote | grep -q "^upstream$"; then
    echo "📌 Adding upstream remote..."
    git remote add upstream https://github.com/openai/codex.git
    echo "✅ Upstream remote added"
fi

# Fetch upstream
echo ""
echo "📥 Fetching from upstream..."
git fetch upstream

# Show current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "Current branch: $CURRENT_BRANCH"

# Merge upstream/main
echo ""
echo "🔀 Merging upstream/main..."
git merge upstream/main --no-ff -m "chore: Sync with upstream openai/codex

Merged latest changes from upstream repository.
Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
User: zapabob"

echo ""
echo "✅ Sync complete!"
echo ""
echo "📋 Next steps:"
echo "1. Resolve conflicts if any"
echo "2. Run tests: npm run test"
echo "3. Build: npm run build"
echo "4. Push: git push origin $CURRENT_BRANCH"