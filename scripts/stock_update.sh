#!/bin/bash

curl -X PUT https://naturalflowersas.azurewebsites.net/stock/update \
    -H "Content-Type: application/json" \
    -d '{
            "id": 1,
            "stripeId": "asdfsadf",
            "name": "not so long",
            "friendlyName": "This is not a Rose",
            "price": 0.01,
            "score": 2,
            "stock": 50, 
            "detainedStock": 20,
            "description": "This description has been changed.",
            "stringQuantities": "1"
    }'
