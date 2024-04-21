#!/bin/bash

./_create_database.sh
./_insert_locale.sh
./_insert_presets.sh

# the following scripts need to be updated with proper filename
./_insert_add_item.sh
./_insert_add_preset.sh
