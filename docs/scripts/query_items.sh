#!/bin/bash

echo "## Items"
echo ""
echo "### Most added items";
echo ""
echo "| Item  | Times |"
echo "|:------|------:|"

sqlite3 ../database/db.sqlite "SELECT il.name || ' ![image](https://assets.tarkov.dev/' || aie.item_id || '-512.webp)' as name, COUNT(*) \"counter\"
                               FROM add_item_event aie
                                        INNER JOIN items_locale il ON il.item_id = aie.item_id
                               GROUP BY aie.item_id
                               ORDER BY \"counter\" DESC limit 20";

echo ""
echo "### Less added items";
echo ""
echo "| Item  | Times |"
echo "|:------|------:|"
sqlite3 ../database/db.sqlite "SELECT il.name || ' ![image](https://assets.tarkov.dev/' || aie.item_id || '-512.webp)' as name, COUNT(*) \"counter\"
                               FROM add_item_event aie
                                        INNER JOIN items_locale il ON il.item_id = aie.item_id
                               GROUP BY aie.item_id
                               ORDER BY \"counter\" ASC limit 20";
