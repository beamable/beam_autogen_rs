#!/bin/bash

if [ ! -f "combinedOpenApi.json" ]; then
    echo "Download api first!"
    exit -1
fi

rm -rf src && rm -rf docs
sed -i 's/"summary"/"description"/g' -i combinedOpenApi.json
jq '.info.title = "Beamable API"' combinedOpenApi.json > new.json
jq '.info.license = {"name": "MIT"}' new.json > combinedOpenApi.json
jq '.info.description = "Autogenerated Beamable API"' combinedOpenApi.json > new.json
rm combinedOpenApi.json
mv new.json combinedOpenApi.json