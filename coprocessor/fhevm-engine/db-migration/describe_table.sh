#!/bin/bash

TABLE_NAME="$1"

psql $DATABASE_URL -P pager=off -c "\d+ $TABLE_NAME"
