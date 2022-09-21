#!/usr/bin/env bash

SERVER_HOST=${SERVER_HOST:-"localhost:8080"}

echo "loop to create 10 players:"
for((i=0;i<10;i++)); \
  do \
    curl --location --request POST "http://${SERVER_HOST}/player/" --header 'Content-Type: application/json' --data-raw '[{"coins":100,"goods":20}]'; \
  done

printf "\n\n"
echo "get player 1:"
curl --location --request GET "http://${SERVER_HOST}/player/1"

printf "\n\n"
echo "get players by limit 3:"
curl --location --request GET "http://${SERVER_HOST}/player/limit/3"

printf "\n\n"
echo "get players count:"
curl --location --request GET "http://${SERVER_HOST}/player/count"

printf "\n\n"
echo "trade by two players:"
curl --location --request PUT "http://${SERVER_HOST}/player/trade" \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'sellID=1' \
  --data-urlencode 'buyID=2' \
  --data-urlencode 'amount=10' \
  --data-urlencode 'price=100'
