#!/bin/sh

#chown vlc:video /dev/video0

#/usr/bin/sudo -u vlc cvlc v4l2:///dev/video0 \
#  --v4l2-width 1920 --v4l2-height 1080 --v4l2-chroma h264 \
#  --sout '#standard{access=http,mux=ts,dst=0.0.0.0:5858}' -vvv

/usr/bin/sudo -u vlc raspivid -o - -t 0 -hf -w 1024 -h 768 -fps 25 -fl --nopreview | \
  sudo -u vlc cvlc -v stream:///dev/stdin --sout '#standard{access=http,mux=ts,dst=:5858}' :demux=h264

# end