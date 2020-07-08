FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

COPY ./target/${TARGETPLATFORM}/release/api-http /

ENTRYPOINT ["/api-http"]
# end
