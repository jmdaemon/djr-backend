#!/bin/bash

## Customer

echo "TEST: Get all Customers (empty)"
# curl -s http://localhost:8080/api/customers
printf "\n"

echo "TEST: Create new Customer"

curl -s -X POST -H "Content-Type: application/json" -d \
    '{
        "first_name": "John",
        "last_name": "Doe",
        "phone": "604-111-1234",
        "email": "john.doe@gmail.com"
    }' http://localhost:8080/api/customers
printf "\n"

echo "TEST: Get all Customers"
curl -s http://localhost:8080/api/customers 
printf "\n"

echo "TEST: Update a Customer by ID"
# id=$(curl -s http://localhost:8080/api/customers | jq '.[0].customer_id' | xargs)
# printf "ID: %s\n" "$id"

# curl -s -X PUT -H "Content-Type: application/json" \
#     http://localhost:8080/api/customers/$id -d \
#     '{
#         "first_name": "John",
#         "last_name": "Dill",
#         "phone": "604-111-1234",
#         "email": "john.doe@gmail.com"
#     }' | jq
printf "\n"

echo "TEST: Delete a Customer by id"
# id=$(curl -s http://localhost:8080/api/customers | jq '.[0].customer_id' | xargs)
# curl -s -X DELETE http://localhost:8080/api/customers/$id | jq
printf "\n"

exit;

## TODO:

echo "TEST: Create a new Customer"
# curl -s -X POST -H "Content-Type: application/json" -d '{"title": "Buy milk", "description": "Buy 2 liters of milk"}' http://localhost:8080/api/todos | jq
# curl -s -X POST -H "Content-Type: application/json" -d '{"title": "Buy eggs", "description": "Buy 12 eggs"}' http://localhost:8080/api/todos | jq
# curl -s -X POST -H "Content-Type: application/json" -d '{"title": "Buy bread", "description": "Buy 1 loaf of bread"}' http://localhost:8080/api/todos | jq

echo "TEST: Get all Customer"
# curl -s http://localhost:8080/api/todos | jq 

# echo "TEST: Get a Todo item by id"
# id=$(curl -s http://localhost:8080/api/todos | jq '.[0].id' | xargs)
# curl -s http://localhost:8080/api/todos/$id | jq

echo "TEST: Update a Customer by id"
# id=$(curl -s http://localhost:8080/api/todos | jq '.[0].id' | xargs)
# curl -s -X PUT -H "Content-Type: application/json" -d '{"title": "Buy 2 liters of milk", "description": "Buy 2 liters of milk"}' http://localhost:8080/api/todos/$id | jq

echo "TEST: Delete a Customer by id"
# id=$(curl -s http://localhost:8080/api/todos | jq '.[0].id' | xargs)
# curl -s -X DELETE http://localhost:8080/api/todos/$id | jq

echo "TEST: Get all Customers"
# curl -s http://localhost:8080/api/todos | jq

