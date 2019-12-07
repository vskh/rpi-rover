FROM alpine:latest
LABEL description="WiFi access point using HostAP"

ARG country_code=IE
ARG interface=wlan0
ARG ssid=Rpi-Rover
ARG wpa_passphrase=rp1rover
ARG ap_address=192.168.33.1
ARG ap_netmask=255.255.255.0

ENV COUNTRY_CODE ${country_code}
ENV INTERFACE ${interface}
ENV SSID ${ssid}
ENV WPA_PASSPHRASE ${wpa_passphrase}

ENV AP_ADDRESS ${ap_address}
ENV AP_NETMASK ${ap_netmask}

RUN apk add hostapd
RUN apk add dnsmasq

COPY . /wifiap
WORKDIR /wifiap

CMD ["/wifiap/setup.sh"]