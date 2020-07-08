FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

COPY ./app /app

EXPOSE 80/tcp
WORKDIR /app
ENTRYPOINT ["/app/api-http"]
# end
