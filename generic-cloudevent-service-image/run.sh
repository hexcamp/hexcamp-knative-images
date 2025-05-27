#! /bin/sh

docker rm -f generic-cloudevent-service

docker run \
  -it \
  -p 10080:8080 \
  -v ./test-fs:/data \
  --name generic-cloudevent-service \
  -e PATH=/cnb/process:/cnb/lifecycle:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/data \
  --entrypoint /cnb/process/function \
  ghcr.io/hexcamp/hexcamp-knative-images/generic-cloudevent-service-image
