#!/bin/sh

gen_config()
{
    TEMPLATE_FILE=$1
    shift

    TEMPLATE=`cat $TEMPLATE_FILE`

    for var in "$@";
    do
        eval "val=\$${var}"
        TEMPLATE=`echo "$TEMPLATE" | sed "s/\\${${var}}/${val}/g"`
    done

    echo "$TEMPLATE"
}

sig_handler() {
    SIGNO=$1
    echo "Received signal ${SIGNO}"
    for pid in ${PIDS};
    do
        kill -${SIGNO} ${pid}
        wait ${pid}
    done

    exit $((128+${SIGNO}))
}

wait_pids() {
    for pid in ${PIDS};
    do
        echo "Awaiting while PID ${pid} is running..."
        wait ${pid}
    done
}

PIDS=

ifconfig wlan0 inet ${AP_ADDRESS} netmask ${AP_NETMASK} up

AP_NETWORK=`ipcalc -n ${AP_ADDRESS} ${AP_NETMASK} | sed 's/NETWORK=//g'`
AP_BROADCAST=`ipcalc -b ${AP_ADDRESS} ${AP_NETMASK} | sed 's/BROADCAST=//g'`

AP_ADDRESS_MIN_BASE=`echo ${AP_ADDRESS} | sed 's/\.[0-9]\+$//g'`
AP_ADDRESS_MIN=`echo ${AP_ADDRESS} | sed 's/.*\.\([0-9]\+\)$/\1/g'`
AP_ADDRESS_MAX_BASE=`echo ${AP_BROADCAST} | sed 's/\.[0-9]\+$//g'`
AP_ADDRESS_MAX=`echo ${AP_BROADCAST} | sed 's/.*\.\([0-9]\+\)$/\1/g'`

export AP_ADDRESS_RANGE_MIN="${AP_ADDRESS_MIN_BASE}.$((${AP_ADDRESS_MIN}+1))"
export AP_ADDRESS_RANGE_MAX="${AP_ADDRESS_MAX_BASE}.$(($AP_ADDRESS_MAX-1))"

HOSTAPD_CONF=`gen_config hostapd.template.conf COUNTRY_CODE INTERFACE SSID WPA_PASSPHRASE`
echo "$HOSTAPD_CONF" > hostapd.conf

DNSMASQ_CONF=`gen_config dnsmasq.template.conf AP_ADDRESS AP_ADDRESS_RANGE_MIN AP_ADDRESS_RANGE_MAX`
echo "$DNSMASQ_CONF" > dnsmasq.conf

trap sig_handler SIGTERM
trap sig_handler SIGINT

/usr/sbin/hostapd -P /wifiap/hostapd.pid /wifiap/hostapd.conf &
HOSTAPD_PID=$!
HOSTAPD_RV=$?

if [ $HOSTAPD_RV -ne 0 ];
then
    echo "Failed to launch hostapd."
    exit 1
fi

echo "Launched hostapd with PID $HOSTAPD_PID."

PIDS="$PIDS $HOSTAPD_PID"

/usr/sbin/dnsmasq -d --conf-file=/wifiap/dnsmasq.conf &
DNSMASQ_PID=$!
DNSMASQ_RV=$?

if [ $DNSMASQ_RV -ne 0 ];
then
    echo "Failed to launch dnsmasq."
    exit 1
fi

echo "Launched dnsmasq with PID $DNSMASQ_PID."

PIDS="$PIDS $DNSMASQ_PID"

echo "Launched WiFi Access Point."

wait_pids

echo "WiFi Access Point shut down."

# end