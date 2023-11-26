#!/bin/bash

echo what comment would you like to leave?:
read comment

echo what star rating would you like to give this item?:
read rating

echo what itemId is this review for?
read item_id

curl -X POST https://naturalflowersas.azurewebsites.net/review \
-H 'Content-Type: application/json' \
-d '{ 
    "comment": "'"$comment"'",
    "rating": '$rating',
    "userId": "24b39d28-11b2-4600-964b-f83bb9965aad",
    "itemId": '$item_id'
}'
