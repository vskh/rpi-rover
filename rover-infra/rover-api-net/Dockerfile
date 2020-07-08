FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

COPY ./app /app

EXPOSE 5757/tcp

ENTRYPOINT ["/app/api-net"]
# end
