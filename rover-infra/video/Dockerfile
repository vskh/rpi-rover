FROM vskhimages/docker-raspberry-pi-runner:latest

RUN apt install -y vlc
RUN useradd -mr -G video vlc

EXPOSE 5858/tcp

COPY ./run.sh /run.sh

ENTRYPOINT ["/run.sh"]

# end