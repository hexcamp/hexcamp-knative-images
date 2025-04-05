#! /bin/bash

curl -v \
  -d '{"name": "Bootsy"}' \
  -H 'content-type: application/json' \
  -H 'ce-specversion: 1.0' \
  -H 'ce-id: 1' \
  -H 'ce-source: http://cloudevents.io' \
  -H 'ce-type: dev.knative.example' \
  http://127.0.0.1:8080

