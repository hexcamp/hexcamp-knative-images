#! /bin/bash

. ../../.env
export CR_PAT

export FUNC_REGISTRY=ghcr.io/hexcamp

#func deploy -v -n repair \
#  -b=s2i --platform linux/amd64

func build -v 
