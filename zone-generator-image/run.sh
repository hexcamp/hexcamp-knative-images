#! /bin/sh

docker rm -f zone-generator

docker run \
  -it \
  -p 8080:8080 \
  -v ./test-fs:/data \
  --name zone-generator \
  -e PATH=/cnb/process:/cnb/lifecycle:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/data \
  ghcr.io/hexcamp/hexcamp-knative-images/zone-generator-image
