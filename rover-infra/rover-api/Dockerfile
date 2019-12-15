FROM raspbian/stretch:latest

ARG TARGETPLATFORM=arm-unknown-linux-gnueabihf

RUN apt update
RUN apt install -y libraspberrypi-bin

COPY ./target/${TARGETPLATFORM}/release/api-server /

ENTRYPOINT ["/api-server"]
# end
