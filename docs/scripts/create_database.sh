#!/bin/bash

echo "Creating database under ../database/db.sqlite";

cmds="create table items_locale
      (
          item_id text PRIMARY KEY,
          name text
      );
      create table add_item_event
      (
          timestamp        text,
          user_id          text,
          session_id       text,
          event_name       text,
          item_id          text,
          numeric_props    text,
          os_name          text,
          os_version       text,
          locale           text,
          app_version      text,
          app_build_number text,
          engine_name      text,
          engine_version   text,
          country_code     text,
          country_name     text,
          region_name      text,
          FOREIGN KEY (item_id) REFERENCES items_locale(item_id)
      );
      ";

echo "$cmds" | sqlite3 ../database/db.sqlite


echo "Done";
