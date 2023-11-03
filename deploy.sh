#!/usr/bin/bash
SSH="vm@192.168.122.32"
cargo build
# ssh $SSH sudo rm -rf /var/lib/extensions/chromebook-audio
# scp -r chromebook-ucm-conf $SSH:
# scp -r conf $SSH
# scp target/debug/setup $SSH:
scp target/debug/service $SSH:
# ssh $SSH sudo ./setup
scp chromebook-audio.service $SSH:
ssh $SSH sudo cp chromebook-audio.service /etc/systemd/system
ssh $SSH sudo systemctl enable chromebook-audio.service
ssh $SSH sudo cp ./service /usr/local/bin/chromebook-audio
ssh $SSH sudo chromebook-audio
