#!/bin/sh -e
# this must be copied to the canvas test machine and ran
# wait until Canvas restart. It can take a long time.
echo 'NODE_TLS_REJECT_UNAUTHORIZED="0"' >> /var/www/canvas-rce-api/.env
service apache2 restart
