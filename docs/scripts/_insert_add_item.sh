#!/bin/bash

# $1: Full path of the monthly csv file

HOME=$(dirname $0);

grep "add_item" $1 | sed 's|"{""item_id"":""\([^""]*\)""}"|\1|g' | sqlite3 $HOME/../database/db.sqlite ".import --csv /dev/stdin add_item_event"
