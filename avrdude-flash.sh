#!/usr/bin/env bash

# https://gist.github.com/nooges/93560cb0c456ade5b530e95892b5e25b
# Thank you Nooges 

MCU=atmega32u4

if grep -q -s Microsoft /proc/version; then
    echo 'ERROR: Pro Micros can not be flashed within the Windows Subsystem for Linux (WSL) currently. Instead, take the .hex file generated and flash it using AVRDUDE, AVRDUDESS, or XLoader.'
    exit 1
fi

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <path to .hex file>"
    exit 1
fi

ls /dev/tty* > /tmp/1
echo -n "Detecting Pro Micro port, reset your Pro Micro now."
while [ -z $USB ]; do
    sleep 1
    echo -n "."
    ls /dev/tty* > /tmp/2
    USB=`diff /tmp/1 /tmp/2 | grep -o '/dev/tty.*'`
done;
echo ""
echo "Detected Pro Micro port at $USB"
# sleep 0.5
avrdude -p $MCU -c avr109 -P $USB -U flash:w:$1