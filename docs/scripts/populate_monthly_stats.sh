#!/bin/bash

HOME=$(dirname $0);

read -p "Full path location of the monthly csv file: " CSV
$HOME/_insert_add_item.sh $CSV
$HOME/_insert_add_preset.sh $CSV

read -p "Filename to be created in docs/tarkov-stash-docs/content/stats, ie: april-2024.md: " MONTHLY_STATE_FILE
read -p "Month title, ie: April 2024: " TITLE
read -p "Date of post, ie: 2024-05-01: " DATE

MONTHLY_STATE_FULL_PATH_FILE=$HOME/../tarkov-stash-docs/content/stats/$MONTHLY_STATE_FILE

echo "+++" >> $MONTHLY_STATE_FULL_PATH_FILE
echo "title = \"$TITLE\""  >> $MONTHLY_STATE_FULL_PATH_FILE;
echo "date = \"$DATE\""  >> $MONTHLY_STATE_FULL_PATH_FILE;
echo "+++" >> $MONTHLY_STATE_FULL_PATH_FILE
echo "" >> $MONTHLY_STATE_FULL_PATH_FILE

$HOME/_query_items.sh  >> $MONTHLY_STATE_FULL_PATH_FILE
$HOME/_query_preset.sh >> $MONTHLY_STATE_FULL_PATH_FILE

echo "File created: $MONTHLY_STATE_FULL_PATH_FILE"




