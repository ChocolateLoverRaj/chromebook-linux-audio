#!/usr/bin/bash
SSH="vm@192.168.122.32"
cargo build --release
cargo generate-rpm
scp target/generate-rpm/chromebook-audio* $SSH:
ssh $SSH sudo rpm-ostree remove chromebook-audio
ssh $SSH sudo rpm-ostree install chromebook-audio*.rpm

# scp target/release/service $SSH:
# ssh $SSH sudo ./service
# ssh $SSH systemd-sysext
ssh $SSH sudo reboot
