#!/bin/sh

chown vlc:video /dev/video0

/usr/bin/sudo -u vlc cvlc v4l2:///dev/video0 \
  --v4l2-width 1920 --v4l2-height 1080 --v4l2-chroma h264 \
  --sout '#standard{access=httpmux=tsdst=0.0.0.0:5858}' -vvv

# end