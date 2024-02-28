FROM rust:alpine3.19 AS builder

ENV RUSTFLAGS='--cfg tokio_unstable -C link-arg=-s'
WORKDIR /opt/build
COPY . /opt/build/
RUN apk update &&\
    apk upgrade &&\
    apk add pkgconfig libressl-dev build-base
RUN wget -O - https://github.com/jemalloc/jemalloc/releases/download/5.2.1/jemalloc-5.2.1.tar.bz2 | tar -xj && \
    cd jemalloc-5.2.1 && \
    ./configure && \
    make && \
    make install
RUN cargo build --release


FROM alpine:3.19

EXPOSE 8080
ENV JEMALLOC_SYS_WITH_MALLOC_CONF=background_thread:true,narenas:1,tcache:false,dirty_decay_ms:0,muzzy_decay_ms:0,abort_conf:true
WORKDIR /opt/app
RUN apk update && apk upgrade
RUN adduser --disabled-password --gecos '' pinger
USER pinger:pinger
COPY --from=builder /opt/build/target/release/pinger .
CMD [ "./pinger" ]
