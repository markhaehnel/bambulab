#!/bin/bash

# This script is used to prepare a release of the project.

# Exit on error
set -e

echo "Preparing release..."

# Bump version
echo "Bumping version..."
cargo semver-bump

# Read version from Cargo.toml
VERSION=$(cat Cargo.toml | grep version | head -n 1 | cut -d '"' -f 2)
BRANCH_NAME=chore/release-${VERSION}

# Generate changelog
echo "Generating changelog..."
git cliff -o CHANGELOG.md --tag "${VERSION}"

git checkout -b "${BRANCH_NAME}"

echo ""
echo "Release prepared! Please commit the changes using the following command:"
echo "git add CHANGELOG.md Cargo.toml Cargo.lock && git commit -m \"chore: release ${VERSION}\" && git push --set-upstream origin ${BRANCH_NAME}"
echo ""
echo "After that, please create a pull request and merge it and then create a new tag using the following command:"
echo "git switch main && git pull && git tag -a ${VERSION} -m \"${VERSION}\" && git push --tags"