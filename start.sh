#!/bin/bash

dev=$(sudo fdisk -l | grep 'My Passport' -B1 | cut -f 2 -d' ' | tr -d ':' | head -n1)
sudo apfs-fuse "${dev}2" ~/media/usb
docker compose up
