# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

ARG BASE_IMAGE=rust:alpine

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

RUN apk update
RUN apk add --no-cache openssl-dev musl-dev perl build-base

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo install --path .


# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM alpine:latest

ARG APP=/promptpay

EXPOSE 8080

ENV TZ=Asia/Bangkok \
    APP_USER=appuser \
    RUST_LOG="debug"

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER \
    && mkdir -p ${APP}/static \

RUN apk update \
  && apk --no-cache add ca-certificates \
  && apk add curl openssl-dev libc-dev zlib-dev libc6-compat\
  && rm -rf /var/cache/apk/*ls


RUN openssl s_client -connect southeastasia-1.in.applicationinsights.azure.com:443 -showcerts </dev/null 2>/dev/null | sed -e '/-----BEGIN/,/-----END/!d' | tee "/usr/local/share/ca-certificates/ca.crt" >/dev/null && \
update-ca-certificates

COPY --from=builder /usr/local/cargo/bin/prompt_pay_service ${APP}/prompt_pay_service

ADD ./static/index.html ${APP}/static/index.html

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./prompt_pay_service"]
