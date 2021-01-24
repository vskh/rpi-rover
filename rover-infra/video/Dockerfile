FROM vskhimages/docker-raspberry-pi-runner:latest

RUN apt install -y vlc
RUN useradd -mr -G video vlc && \
    echo "vlc ALL = (root) NOPASSWD:/usr/bin/vlc-wrapper" > /etc/sudoers.d/vlc && \
    chown vlc:video /dev/video0

USER vlc

EXPOSE 5858/tcp

ENTRYPOINT ["/usr/bin/sudo", \
            "/usr/bin/vlc-wrapper", "-Idummy", "v4l2:///dev/video0", \
            "--v4l2-width", "1920", "--v4l2-height", "1080", "--v4l2-chroma", "h264", \
            "--sout", "#standard{access=http,mux=ts,dst=0.0.0.0:5858}", \
            "-vvv"]
# end