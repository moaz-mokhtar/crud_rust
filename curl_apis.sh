#!/usr/bin/env bash

# set -xeu

PORT=8080

echo "\n\n==========================="
echo "Healthcheck"
curl -X GET http://localhost:$PORT/api/


echo "\n\n==========================="
echo "Get all drivers"
curl -X GET http://localhost:$PORT/api/drivers


echo "\n\n==========================="
echo "Create new driver"
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"first_name": "Moaz","last_name": "Mokhtar", "email": "moaz.mokhtar@gmail.com", "phone": "0154864354"}'
    http://localhost:$PORT/api/drivers



echo "\n\n==========================="
echo "Get all drivers"
curl -X GET http://localhost:$PORT/api/drivers


# echo "\n\n==========================="
# echo "Get a driver"
# curl http://localhost:$PORT/api/drivers/0


# echo "\n\n==========================="
# echo "Delete a driver"
# curl -X DELETE http://localhost:$PORT/api/drivers/0
