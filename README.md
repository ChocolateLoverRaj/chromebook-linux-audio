<h1 align="center">Python script to enable audio support on Chrome devices</h1>

# This is a fork
If you want audio on a mutable Linux distro, use https://github.com/WeirdTreeThing/chromebook-linux-audio
If this isn't working for some reason and you want audio on an immutable Linux distro, use https://github.com/ChocolateLoverRaj/chromebook-linux-audio

# Instructions
Download the RPM from GitHub releases and install.

See the [Linux compatibility sheet](https://docs.google.com/spreadsheets/d/1udREts28cIrCL5tnPj3WpnOPOhWk76g3--tfWbtxi6Q/edit#gid=0) for more info.

# Officially Supported distros
Fedora 38 & immutable spins (Silverblue, Kinoite, etc.)

# Developing
## Setup the environment
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [`virt-manager`](https://virt-manager.org/)
- Create a Fedora Kinoite (or other immutable spin) VM
- Enable SSH in the VM and setup SSH so you don't need to type in a password
- Disable password for the VM user
- Edit `deploy.sh` to use your VM's username and IP address

## Testing changes in the VM
Run `deploy.sh`

## Testing changes on a real device
The easiest way is to run `build.sh`, which will generate a `.rpm` file in `target/generate-rpm`. Then just install that RPM.
