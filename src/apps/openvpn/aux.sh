#!/bin/sh
# this must be copied to the openvpn test machine and ran
openvpn-addclient client client@example.com
/var/www/openvpn/bin/addprofile client | sed 's/^URL: //'
