#!/bin/bash
# Fetch the list of changed files
CHANGED_FILES=$(git diff --name-only ${{ github.event.before }} ${{ github.sha }})
PROVIDERS_DIR="src/providers/"

# Iterate over each changed file
for file in $CHANGED_FILES; do
    # Check if the file is in the PROVIDERS_DIR
    if [[ $file == $PROVIDERS_DIR* ]]; then
        # Extract the specific test name from the file path
        PROVIDER_TEST_NAME=$(echo $file | sed "s|^$PROVIDERS_DIR||" | sed 's|/|::|g' | sed 's|\.rs$||')_provider
        # Run specific test for that file
        echo -n "$PROVIDER_TEST_NAME "
    fi
done
