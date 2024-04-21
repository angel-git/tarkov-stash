#!/bin/bash

HOME=$(dirname $0);

cat $HOME/../../example/Aki_Data/Server/database/locales/global/en.json | jq -c 'with_entries(select(.key | test("[a-zA-Z0-9]{24} Name")))' | jq -r 'to_entries[] |
  {item_id: .key|split(" ")[0], name: .value} |
  [.[]] |
  @csv' | sqlite3 $HOME/../database/db.sqlite ".import --csv /dev/stdin items_locale"

