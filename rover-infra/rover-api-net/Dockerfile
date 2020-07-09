FROM vskhimages/docker-raspberry-pi-runner:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

COPY ./app /app

RUN apt install -y gdb

EXPOSE 5757/tcp
WORKDIR /app
ENTRYPOINT ["/app/api-net"]
# end
