#!/bin/bash

echo "What is the id of the item you would like to delete?: "
read id

curl -X DELETE https://naturalflowersas.azurewebsites.net/stock/$id
