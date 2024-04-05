# Documentation

## Database

Run the scripts from within `scripts` folder to create the database.

### add_item event

- get the most added items:

```sqlite
SELECT aie.item_id, il.name, COUNT(*) "counter"
FROM add_item_event aie
         INNER JOIN items_locale il ON il.item_id = aie.item_id
GROUP BY aie.item_id
ORDER BY "counter" DESC
```

## Docs site

TODO
