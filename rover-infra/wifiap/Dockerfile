FROM alpine:latest
LABEL description="WiFi access point using HostAP"

ARG country_code=IE
ARG interface=wlan0
ARG ssid=Rpi-Rover
ARG wpa_passphrase=rp1rover

ENV COUNTRY_CODE ${country_code}
ENV INTERFACE ${interface}
ENV SSID ${ssid}
ENV WPA_PASSPHRASE ${wpa_passphrase}

RUN apk add hostapd
RUN mkdir /wifiap
COPY . /wifiap
WORKDIR /wifiap
RUN /wifiap/setup.sh

CMD ["/usr/sbin/hostapd", "/wifiap/hostapd.conf"]