#!/bin/bash

# Build the project
echo "Building project..."
trunk build --release --public-url "/restoration_project/"

# Check if dist directory exists
if [ ! -d "dist" ]; then
    echo "Error: dist directory not found. Build may have failed."
    exit 1
fi

echo "Build complete! Files are in the 'dist' directory."
echo ""
echo "To deploy to GitHub Pages:"
echo "1. Create a 'gh-pages' branch in your repository"
echo "2. Copy the contents of 'dist' to the gh-pages branch"
echo "3. Commit and push the gh-pages branch"
echo ""
echo "Or use the GitHub Actions workflow for automatic deployment."