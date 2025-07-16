#!/bin/bash

echo "TEST: Create a new Todo items"
# curl -s -X POST -H "Content-Type: application/json" -d '{"title": "Buy milk", "description": "Buy 2 liters of milk"}' http://localhost:8080/api/todos | jq
# curl -s -X POST -H "Content-Type: application/json" -d '{"title": "Buy eggs", "description": "Buy 12 eggs"}' http://localhost:8080/api/todos | jq
# curl -s -X POST -H "Content-Type: application/json" -d '{"title": "Buy bread", "description": "Buy 1 loaf of bread"}' http://localhost:8080/api/todos | jq

echo "TEST: Get all Todo items"
# curl -s http://localhost:8080/api/todos | jq 

# echo "TEST: Get a Todo item by id"
# id=$(curl -s http://localhost:8080/api/todos | jq '.[0].id' | xargs)
# curl -s http://localhost:8080/api/todos/$id | jq

echo "TEST: Update a Todo item by id"
# id=$(curl -s http://localhost:8080/api/todos | jq '.[0].id' | xargs)
# curl -s -X PUT -H "Content-Type: application/json" -d '{"title": "Buy 2 liters of milk", "description": "Buy 2 liters of milk"}' http://localhost:8080/api/todos/$id | jq

echo "TEST: Delete a Todo item by id"
# id=$(curl -s http://localhost:8080/api/todos | jq '.[0].id' | xargs)
# curl -s -X DELETE http://localhost:8080/api/todos/$id | jq

echo "TEST: Get all Todo items"
curl -s http://localhost:8080/api/todos | jq
