#!/bin/bash

HOME_DIR=$(dirname "$0")

# Function to fetch releases for a specific page
fetch_page_of_releases() {
    local page_number="$1"
    local response
    response=$(curl -s "https://api.github.com/repos/angel-git/tarkov-stash/releases?page=$page_number")
    echo "$response"
}

# Initialize variables
page=1
markdown=""

# Loop to fetch all releases
while true; do
    response=$(fetch_page_of_releases "$page")

    # Check if response is empty or not
    if [ -z "$response" ]; then
        echo "Failed to retrieve data from GitHub API"
        exit 1
    fi

    # Parse JSON response and format as markdown
    releases=$(echo "$response" | jq -r '.[] | "## Release: \(.tag_name) @ \(.created_at)\\n```\\n\(.body)\\n```\\n\\n"')
    markdown="$markdown$releases"

    # Check if there are more pages
    if [ "$(echo "$response" | jq length)" -eq 0 ]; then
        break
    fi

    # Move to the next page
    ((page++))
done

# Write markdown to file
CHANGELOG_FILE="$HOME_DIR/../tarkov-stash-docs/content/changelog.md"
echo "+++" > "$CHANGELOG_FILE"
echo 'title = ""' >> "$CHANGELOG_FILE"
echo "+++" >> "$CHANGELOG_FILE"
echo ""  >> "$CHANGELOG_FILE"
echo -e "$markdown" >> "$CHANGELOG_FILE"
