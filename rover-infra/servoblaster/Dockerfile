FROM raspbian/stretch:latest AS builder

COPY ./src /servoblaster
COPY ./locations.patch /servoblaster

WORKDIR /servoblaster

RUN apt update && apt upgrade -y
RUN apt install -y build-essential libraspberrypi-dev

RUN patch < locations.patch
RUN make

FROM scratch
LABEL description="ServoBlaster service for servomotors control"

COPY --from=builder /servoblaster/servod /

ENTRYPOINT ["/servod"]

# end