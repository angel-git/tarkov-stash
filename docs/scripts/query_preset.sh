#!/bin/bash

echo "## Presets"
echo ""
echo "### Most added presets";
echo ""
echo "| Item  | Times |"
echo "|:------|------:|"

sqlite3 ../database/db.sqlite "SELECT  il.name || ' ![image](https://assets.tarkov.dev/' || p.item_id || '-512.webp)' as name,  COUNT(*) "counter"
                               FROM add_preset_event aie
                                        INNER JOIN presets p ON p.item_id = aie.item_id
                                        INNER JOIN items_locale il ON p.encyclopedia = il.item_id
                               GROUP BY aie.item_id
                               ORDER BY "counter" DESC limit 20;";
echo ""
echo "### Less added presets";
echo ""
echo "| Item  | Times |"
echo "|:------|------:|"
sqlite3 ../database/db.sqlite "SELECT  il.name || ' ![image](https://assets.tarkov.dev/' || p.item_id || '-512.webp)' as name,  COUNT(*) "counter"
                               FROM add_preset_event aie
                                        INNER JOIN presets p ON p.item_id = aie.item_id
                                        INNER JOIN items_locale il ON p.encyclopedia = il.item_id
                               GROUP BY aie.item_id
                               ORDER BY "counter" ASC limit 20;";
