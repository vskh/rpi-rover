FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGET_PLATFORM=arm-unknown-linux-gnueabihf

COPY ./app /app

EXPOSE 80/tcp
WORKDIR /app
ENTRYPOINT ["/app/api-http"]
# end
