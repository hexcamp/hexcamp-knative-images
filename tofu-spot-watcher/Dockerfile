FROM alpine

RUN echo @edge https://dl-cdn.alpinelinux.org/alpine/edge/community >> /etc/apk/repositories
RUN echo @edge https://dl-cdn.alpinelinux.org/alpine/edge/main >> /etc/apk/repositories
RUN echo @edge https://dl-cdn.alpinelinux.org/alpine/edge/testing >> /etc/apk/repositories
RUN apk -U upgrade
RUN apk add --no-cache git opentofu aws-cli

WORKDIR /work

ENTRYPOINT ["sh"]
