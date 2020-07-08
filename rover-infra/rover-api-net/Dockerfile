FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

COPY ./build/api-net /

ENTRYPOINT ["/api-net"]
# end
