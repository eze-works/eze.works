#!/usr/bin/env sh

MIX_ENV=prod mix release
cp deploy/eze_works.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user restart eze_works.service
systemctl --user status eze_works.service
