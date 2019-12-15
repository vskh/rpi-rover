FROM raspbian/stretch:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

COPY ./target/${TARGETPLATFORM}/release/api-server /

ENTRYPOINT ["/api-server"]
# end
