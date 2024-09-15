#!/usr/bin/env sh

set -euo pipefail

rm -rf tmp/ezeweb/
mkdir -p tmp/ezeweb

# Create the deployment

cp -r assets tmp/ezeweb/assets
cp -r posts tmp/ezeweb/posts


cargo build --release
cp target/release/ezeweb tmp/ezeweb/ezeweb 


# Create an archive
tar --create --gzip --file tmp/ezeweb.tar.gz --directory tmp/ ezeweb


# Copy the archive & systemd unit file to the remote machine
scp ./deploy/ezeweb.service "${USER}@${HOST}:~/.config/systemd/user/"
scp ./tmp/ezeweb.tar.gz "${USER}@${HOST}:~/deploy/"

# Remote to the machine, extract the archive and restart the systemd service
echo "
set -euxo pipefail
tar --extract --gzip --file ~/deploy/ezeweb.tar.gz --directory ~/deploy
systemctl --user daemon-reload
systemctl --user restart ezeweb.service
" |  ssh -T "${USER}@${HOST}"
