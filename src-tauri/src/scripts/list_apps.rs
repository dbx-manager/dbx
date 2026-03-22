pub static  listappsScript:String ="#!/bin/bash

if command -v apt &> /dev/null; then
    apt list --installed 2>/dev/null | cut -d'/' -f1
elif command -v dnf &> /dev/null; then
    dnf list installed 2>/dev/null | tail -n +2 | awk '{print $1}'
elif command -v yum &> /dev/null; then
    yum list installed 2>/dev/null | tail -n +2 | awk '{print $1}'
elif command -v pacman &> /dev/null; then
    pacman -Qq
elif command -v zypper &> /dev/null; then
    zypper search --installed-only 2>/dev/null | awk '{print $3}'
elif command -v apk &> /dev/null; then
    apk info --installed
else
    echo 'No supported package manager found.'
    exit 1
fi";