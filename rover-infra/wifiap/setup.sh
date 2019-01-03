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
    for pid in $PIDS;
    do
        kill -$SIGNO "$pid"
        wait "$pid"
    done

    exit $((128+$SIGNO))
}

wait_pids() {
    for pid in $PIDS;
    do
        echo "Awaiting while PID $pid is running..."
        wait "$pid"
    done
}

PIDS=

ifconfig wlan0 inet ${AP_ADDRESS} netmask ${AP_NETMASK} up
AP_ADDRESS_BASE=`echo $AP_ADDRESS | sed 's/\.[0-9]\+$//g'`
AP_ADDRESS_LAST=`echo $AP_ADDRESS | sed 's/.*\.\([0-9]\)\+$/\1/g'`

export AP_ADDRESS_RANGE_MIN="$AP_ADDRESS_BASE.$(($AP_ADDRESS_LAST+1))"
export AP_NETWORK=`ipcalc -n ${AP_ADDRESS} ${AP_NETMASK} | sed 's/NETWORK=//g'`


HOSTAPD_CONF=`gen_config hostapd.template.conf COUNTRY_CODE INTERFACE SSID WPA_PASSPHRASE`
echo "$HOSTAPD_CONF" > hostapd.conf

DHCPD_CONF=`gen_config dhcpd.template.conf AP_ADDRESS AP_ADDRESS_MIN AP_NETMASK AP_NETWORK`
echo "$DHCPD_CONF" > dhcpd.conf
touch /var/lib/dhcp/dhcpd.leases

trap sig_handler SIGTERM

/usr/sbin/hostapd -P /wifiap/hostapd.pid /wifiap/hostapd.conf &
HOSTAPD_RV=$?

if [ $HOSTAPD_RV -ne 0 ];
then
    echo "Failed to launch hostapd."
    exit 1
fi

HOSTAPD_PID=`cat /wifiap/hostapd.pid`

echo "Launched HostAPd with PID $HOSTAPD_PID."

PIDS="$PIDS $HOSTAPD_PID"

/usr/sbin/dhcpd -f -pf /wifiap/dhcpd.pid -cf /wifiap/dhcpd.conf &
DHCPD_RV=$?

if [ $DHCPD_RV -ne 0 ];
then
    echo "Failed to launch dhcpd."
    exit 1
fi

DHCPD_PID=`cat /wifiap/dhcpd.pid`
echo "Launched DHCPD with PID $DHCPD_PID."

PIDS="$PIDS $DHCPD_PID"

echo "Launched WiFi Access Point."

wait_pids

echo "WiFi Access Point shut down."

# end