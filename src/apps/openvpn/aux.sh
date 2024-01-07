#!/bin/sh -e
# this must be copied to the openvpn test machine and ran
openvpn-addclient test test@example.com
/var/www/openvpn/bin/addprofile test
