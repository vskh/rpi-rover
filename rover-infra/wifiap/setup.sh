#!/bin/sh

HOSTAPD_CONF=`cat hostapd.template.conf`

for var in COUNTRY_CODE INTERFACE SSID WPA_PASSPHRASE;
do
    eval "val=\$${var}"
    HOSTAPD_CONF=`echo "$HOSTAPD_CONF" | sed "s/\\${${var}}/${val}/g"`
done

echo "$HOSTAPD_CONF" > hostapd.conf