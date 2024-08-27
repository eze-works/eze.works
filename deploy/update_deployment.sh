#!/usr/bin/env sh

MIX_ENV=prod mix release
cp deploy/eze_works.service ~/.config/systemd/user/
systemctl --user stop eze_works.service
systemctl --user reenable eze_works.service
systemctl --user start eze_works.service
systemctl --user status eze_works.service
