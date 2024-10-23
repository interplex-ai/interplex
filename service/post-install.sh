#!/bin/sh
sed "s/__USERNAME__/$USER/" /etc/systemd/system/interplex.service.template > /etc/systemd/system/interplex.service
systemctl daemon-reload
systemctl enable interplex.service
systemctl start interplex.service
