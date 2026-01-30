#!/bin/bash

cd ..

docker-compose down -v
docker-compose up -d

sqlx db create
sqlx migrate run

cd -
