# syntax=docker/dockerfile:1.4

FROM ghcr.io/hexcamp/zone-generator:latest@sha256:5c6e0fe0cda568720b3e356b525f513bba1152bc03a13ce8c369260ade2248c3 AS func

FROM ubuntu:noble

USER root

RUN apt -y update
RUN apt -y upgrade
RUN apt -y install jq curl git csvkit xz-utils vim rclone
RUN curl -sSfL https://github.com/mitsuhiko/minijinja/releases/latest/download/minijinja-cli-installer.sh | sh

USER 1000

COPY --from=func /layers /layers
COPY --from=func /cnb /cnb
COPY --from=func /workspace /workspace

WORKDIR /workspace
ENTRYPOINT /cnb/process/function

ENV PATH=/cnb/process:/cnb/lifecycle:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/workspace/.local/bin/env
ENV CNB_LAYERS_DIR=/layers
ENV CNB_APP_DIR=/workspace
ENV CNB_PLATFORM_API=0.9
ENV CNB_DEPRECATION_MODE=quiet

USER root
