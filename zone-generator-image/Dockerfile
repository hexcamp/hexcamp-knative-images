# syntax=docker/dockerfile:1.4

FROM ghcr.io/hexcamp/zone-generator@sha256:7173b488bb796013ea822360a18c7d96e2f1a4a80e5481d6346e78129c7cebc5 AS func

FROM ubuntu:noble

USER root

RUN apt -y update
RUN apt -y upgrade
RUN apt -y install jq curl git csvkit xz-utils vim rclone
RUN curl -sSfL https://github.com/mitsuhiko/minijinja/releases/latest/download/minijinja-cli-installer.sh | sh

# https://stackoverflow.com/questions/25899912/how-to-install-nvm-in-docker/60137919#60137919
SHELL ["/bin/bash", "--login", "-i", "-c"]
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
RUN source /root/.bashrc && nvm install 24.12.0
SHELL ["/bin/bash", "--login", "-c"]

ENV PATH=/root/.nvm/versions/node/v24.12.0/bin:$PATH
RUN npm install -g hexcamp-tool

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
