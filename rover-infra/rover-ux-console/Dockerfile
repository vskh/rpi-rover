FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGET_PLATFORM=arm-unknown-linux-gnueabihf

COPY ./app /app

WORKDIR /app
ENTRYPOINT ["/app/ux-console", "--remote", "rover-api-net:5757"]
# end