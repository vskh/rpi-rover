FROM nginx:alpine
LABEL description="API reverse proxy"

COPY ./data /data
COPY ./nginx.conf /etc/nginx/nginx.conf
COPY ./mime.types /etc/nginx/mime.types

# end