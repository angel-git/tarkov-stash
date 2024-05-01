#!/bin/bash

HOME=$(dirname $0);


# Call GitHub API and store response in a variable
response=$(curl -s https://api.github.com/repos/angel-git/tarkov-stash/releases)

# Check if response is empty or not
if [ -z "$response" ]; then
  echo "Failed to retrieve data from GitHub API"
  exit 1
fi

# Parse JSON response and format as markdown
markdown=$(echo "$response" | jq -r '.[] | "## Release: \(.tag_name) @ \(.created_at)\\n```\\n\(.body)\\n```\\n\\n"')


rm $HOME/../tarkov-stash-docs/content/changelog.md

CHANGELOG_FILE=$HOME/../tarkov-stash-docs/content/changelog.md
echo "+++" >> $CHANGELOG_FILE
echo 'title = ""' >> $CHANGELOG_FILE
echo "+++" >> $CHANGELOG_FILE
echo ""  >> $CHANGELOG_FILE

echo -e "$markdown" >> $CHANGELOG_FILE
