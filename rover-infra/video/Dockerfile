FROM vskhimages/docker-raspberry-pi-runner:latest

RUN apt install -y vlc && \
    useradd -mr -G video vlc

RUN echo "vlc ALL = (root) NOPASSWD:/usr/bin/vlc-wrapper" > /etc/sudoers.d/vlc

USER vlc

EXPOSE 5858/tcp

ENTRYPOINT ["/usr/bin/sudo",
            "/usr/bin/vlc-wrapper", "v4l2:///dev/video0",
            "--v4l2-width", "1920", "--v4l2-height", "1080", "--v4l2-chroma", "h264",
            "--sout", "'#standard{access=http,mux=ts,dst=0.0.0.0:12345}'",
            "-vvv"]
# end