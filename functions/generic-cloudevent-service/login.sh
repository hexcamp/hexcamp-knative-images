#! /bin/bash

. ../../.env
export CR_PAT

echo 'Might need to run "security unlock-keychain" first'

echo $CR_PAT | docker login ghcr.io -u USERNAME --password-stdin

