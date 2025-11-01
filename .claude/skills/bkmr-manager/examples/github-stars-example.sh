#!/usr/bin/env bash
# Example: Import and manage GitHub stars workflow

set -euo pipefail

echo "=== GitHub Stars to bkmr Workflow Example ==="
echo ""

# Step 1: Import stars
echo "Step 1: Import GitHub stars"
echo "$ ./scripts/import-github-stars.sh danielbodnar"
echo ""

# Example generated import command
cat << 'EOF'
Example generated import:

# sysid/bkmr
echo "Importing: sysid/bkmr"
bkmr add "https://github.com/sysid/bkmr" \
  github,rust,cli,bookmark,snippet,language-rust,repo-bkmr,author-sysid,status-active,stars-100,year-2025 \
  --title "sysid/bkmr - Fast bookmark manager" \
  --description "Fast bookmark and snippet manager | Homepage: https://sysid.github.io/bkmr-reborn/ | Stars: 127 | Updated: 2025-10-15" || echo "Failed to import sysid/bkmr"

EOF

# Step 2: Enable embeddings
echo "Step 2: Enable embeddings for GitHub stars"
echo '$ for id in $(bkmr search -t github --np); do bkmr set-embeddable $id --enable; done'
echo ""

# Step 3: Backfill embeddings
echo "Step 3: Generate embeddings"
echo "$ bkmr --openai backfill"
echo ""

# Step 4: Fetch READMEs
echo "Step 4: Fetch README files"
echo "$ ./scripts/fetch-readme-embeddings.sh --filter language-rust --limit 50"
echo ""

# Step 5: Search
echo "Step 5: Search semantically"
cat << 'EOF'
$ bkmr --openai sem-search "CLI tools for developers"

Results:
1. [0.94] sysid/bkmr - Fast bookmark manager
2. [0.91] BurntSushi/ripgrep - Fast search
3. [0.89] sharkdp/fd - Fast file finder
...

EOF

# Step 6: Query by category
echo "Step 6: Query by category"
echo "$ bkmr search -t language-rust,cli"
echo "$ bkmr search --fzf -t github,stars-1k"
echo "$ bkmr --openai sem-search \"authentication libraries\" -t language-typescript"
echo ""

echo "=== Complete! ==="
