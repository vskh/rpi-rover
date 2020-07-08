FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

COPY ./app /app

ENTRYPOINT ["/app/api-net"]
# end
