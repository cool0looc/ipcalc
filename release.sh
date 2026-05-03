#!/bin/bash
set -e

# Release script for ipcalc
# Usage: ./release.sh v0.1.0

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v0.1.0"
    exit 1
fi

VERSION="$1"
REPO="cool0looc/ipcalc"

# 检查 gh CLI
if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed"
    echo "Install it from: https://cli.github.com/"
    exit 1
fi

# 检查登录状态
if ! gh auth status &> /dev/null; then
    echo "Please login first: gh auth login"
    exit 1
fi

echo "Creating release ${VERSION} for ${REPO}..."

# 创建 tag (如果不存在)
if ! git rev-parse "${VERSION}" &> /dev/null; then
    echo "Creating git tag ${VERSION}..."
    git tag -a "${VERSION}" -m "Release ${VERSION}"
    git push origin "${VERSION}"
else
    echo "Tag ${VERSION} already exists"
fi

# 等待 CI 完成
echo "Waiting for GitHub Actions to complete..."
gh run list --workflow=release.yml --status=in_progress --limit=1 || echo "No running workflows"

echo ""
echo "Release ${VERSION} has been created!"
echo "Watch progress at: https://github.com/${REPO}/actions"
echo ""
echo "After builds complete, your binaries will be available at:"
echo "https://github.com/${REPO}/releases/${VERSION}"
