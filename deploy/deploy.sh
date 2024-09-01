#!/usr/bin/env sh

rm -rf tmp/release/
mkdir -p tmp/release

# Create the release
MIX_ENV=prod mix release --path tmp/release/eze_works --quiet

# Create an archive
tar --create --gzip --file tmp/release/eze_works.tar.gz --directory tmp/release eze_works


# Copy the archive & systemd unit file to the remote machine
scp ./deploy/eze_works.service "${USER}@${HOST}:~/.config/systemd/user/"
scp ./tmp/release/eze_works.tar.gz "${USER}@${HOST}:~/deploy/"

# Extract the archive on the remote machine
ssh "${USER}@${HOST}" "tar --extract --gzip --file ~/deploy/eze_works.tar.gz --directory ~/deploy && systemctl --user daemon-reload && systemctl --user restart eze_works.service"
