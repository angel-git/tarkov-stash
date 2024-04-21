#!/bin/bash

HOME=$(dirname $0);

cat $HOME/../../example/Aki_Data/Server/database/globals.json | jq -r '.ItemPresets | to_entries[] | [.value["_id"], .value["_encyclopedia"]] | @csv' | sqlite3 $HOME/../database/db.sqlite ".import --csv /dev/stdin presets"

