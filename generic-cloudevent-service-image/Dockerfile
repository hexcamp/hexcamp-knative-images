# syntax=docker/dockerfile:1.4

FROM ghcr.io/hexcamp/generic-cloudevent-service@sha256:644d27f78bb32c1804a88efc619fedb709827ec41023d4250d9317ea1c86c410 AS func

FROM ubuntu:noble

USER root

RUN apt -y update
RUN apt -y upgrade
RUN apt -y install jq curl vim

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
