#!/bin/bash

echo "MOST ADDED ITEMS";

sqlite3 ../database/db.sqlite "SELECT aie.item_id, il.name, COUNT(*) \"counter\"
                               FROM add_item_event aie
                                        INNER JOIN items_locale il ON il.item_id = aie.item_id
                               GROUP BY aie.item_id
                               ORDER BY \"counter\" DESC limit 20";

echo "LESS ADDED ITEMS";
sqlite3 ../database/db.sqlite "SELECT aie.item_id, il.name, COUNT(*) \"counter\"
                               FROM add_item_event aie
                                        INNER JOIN items_locale il ON il.item_id = aie.item_id
                               GROUP BY aie.item_id
                               ORDER BY \"counter\" ASC limit 20";
