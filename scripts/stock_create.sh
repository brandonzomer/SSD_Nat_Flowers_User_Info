#!/bin/bash

url="https://naturalflowersas.azurewebsites.net/stock/add"

curl -X POST $url \
    -H "Content-Type: application/json" \
    -d '{
            "stripeId":	"asdfsadf",
            "name":	"something nice",
            "friendlyName":	"Super cool flower!!!!!",
            "price": 999999.99,
            "score": 999,
            "stock": 9999,
            "detainedStock": 9999,
            "description":	"What kind of flower is this?????",
            "stringQuantities": "123456789"

        }'
