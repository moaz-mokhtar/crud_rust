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
    -d '{"first_name": "Moaz","last_name": "Mokhtar", "email": "moaz.mokhtar@gmail.com", "phone": "0154864354"}' \
    http://localhost:$PORT/api/drivers


echo "\n\n==========================="
echo "Get all drivers"
curl -X GET http://localhost:$PORT/api/drivers


echo "\n\n==========================="
echo "Create 100 drivers"
curl -X GET http://localhost:$PORT/api/drivers/rand100/


echo "\n\n==========================="
echo "Get all drivers"
curl -X GET http://localhost:$PORT/api/drivers/


echo "\n\n==========================="
echo "Get all drivers sorted 'name'"
curl -X GET http://localhost:$PORT/api/drivers/all_by_name/


echo "\n\n==========================="
echo "Get all drivers which names are sorted by characters"
curl -X GET http://localhost:$PORT/api/drivers/all_by_char/


# Uncomment below after add valid driver_uuid

# echo "\n\n==========================="
# echo "Get a driver"
# curl http://localhost:$PORT/api/drivers/0

# echo "\n\n==========================="
# echo "Delete a driver"
# curl -X DELETE http://localhost:$PORT/api/drivers/0
