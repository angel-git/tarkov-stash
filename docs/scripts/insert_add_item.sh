#!/bin/bash

grep "add_item" tarkovstash-release-2024-04-2.csv | sed 's|"{""item_id"":""\([^""]*\)""}"|\1|g' | sqlite3 ../database/db.sqlite ".import --csv /dev/stdin add_item_event"
