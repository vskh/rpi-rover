FROM vskhimages/docker-raspberry-pi-builder:latest AS builder

COPY ./src /servoblaster
COPY ./locations.patch /servoblaster

WORKDIR /servoblaster

RUN patch < locations.patch
RUN make

FROM vskhimages/docker-raspberry-pi-runner:latest AS runner
LABEL description="ServoBlaster service for servomotors control"

COPY --from=builder /servoblaster/servod /

ENTRYPOINT ["/servod"]

# end