#!/bin/bash


cat ../../example/Aki_Data/Server/database/globals.json | jq -r '.ItemPresets | to_entries[] | [.value["_id"], .value["_encyclopedia"]] | @csv' | sqlite3 ../database/db.sqlite ".import --csv /dev/stdin presets"

