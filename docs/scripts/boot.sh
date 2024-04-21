#!/bin/bash

HOME=$(dirname $0);

$HOME/_create_database.sh
$HOME/_insert_locale.sh
$HOME/_insert_presets.sh
