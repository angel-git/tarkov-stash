#!/bin/bash

grep "add_preset" tarkovstash-release-2024-04.csv | sed 's|"{""item_id"":""\([^""]*\)""}"|\1|g' | sqlite3 ../database/db.sqlite ".import --csv /dev/stdin add_preset_event"
