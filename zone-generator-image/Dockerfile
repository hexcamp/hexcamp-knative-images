FROM alpine:3.21.3 AS extra

RUN echo @edge https://dl-cdn.alpinelinux.org/alpine/edge/community >> /etc/apk/repositories
RUN echo @edge https://dl-cdn.alpinelinux.org/alpine/edge/main >> /etc/apk/repositories
RUN echo @edge https://dl-cdn.alpinelinux.org/alpine/edge/testing >> /etc/apk/repositories
RUN apk -U upgrade
RUN apk add --no-cache jq curl

# syntax=docker/dockerfile:1.4

FROM ghcr.io/hexcamp/zone-generator:latest@sha256:5c6e0fe0cda568720b3e356b525f513bba1152bc03a13ce8c369260ade2248c3

USER root
COPY --from=extra /usr/bin/jq                     /usr/bin/jq
COPY --from=extra /usr/lib/libonig.so.5           /usr/lib/libonig.so.5
COPY --from=extra /lib/ld-musl-x86_64.so.1        /lib/ld-musl-x86_64.so.1

COPY --from=extra /usr/bin/curl                   /usr/bin/curl
COPY --from=extra /usr/lib/libcurl.so.4         	/usr/lib/libcurl.so.4                 
COPY --from=extra /usr/lib/libz.so.1            	/usr/lib/libz.so.1
COPY --from=extra /usr/lib/libcares.so.2        	/usr/lib/libcares.so.2
COPY --from=extra /usr/lib/libnghttp2.so.14     	/usr/lib/libnghttp2.so.14
COPY --from=extra /usr/lib/libidn2.so.0         	/usr/lib/libidn2.so.0
COPY --from=extra /usr/lib/libpsl.so.5          	/usr/lib/libpsl.so.5
COPY --from=extra /usr/lib/libssl.so.3          	/usr/lib/libssl.so.3
COPY --from=extra /usr/lib/libcrypto.so.3       	/usr/lib/libcrypto.so.3
COPY --from=extra /usr/lib/libzstd.so.1         	/usr/lib/libzstd.so.1
COPY --from=extra /usr/lib/libbrotlidec.so.1    	/usr/lib/libbrotlidec.so.1
COPY --from=extra /usr/lib/libunistring.so.5    	/usr/lib/libunistring.so.5
COPY --from=extra /usr/lib/libbrotlicommon.so.1 	/usr/lib/libbrotlicommon.so.1
RUN ln -s /lib/ld-musl-x86_64.so.1 /lib/libc.musl-x86_64.so.1

USER 1000

