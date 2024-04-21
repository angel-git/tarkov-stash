#!/bin/bash

echo "TODO!!!"
# presets id don't have locale, need to check encyclopedia....
echo "MOST ADDED PRESETS";

sqlite3 ../database/db.sqlite "SELECT aie.item_id, il.name, COUNT(*) \"counter\"
                               FROM add_preset_event aie
                                        INNER JOIN items_locale il ON il.item_id = aie.item_id
                               GROUP BY aie.item_id
                               ORDER BY \"counter\" DESC limit 20";

echo "LESS ADDED PRESETS";
sqlite3 ../database/db.sqlite "SELECT aie.item_id, il.name, COUNT(*) \"counter\"
                               FROM add_preset_event aie
                                        INNER JOIN items_locale il ON il.item_id = aie.item_id
                               GROUP BY aie.item_id
                               ORDER BY \"counter\" ASC limit 20";
