#!/usr/bin/bash
UNIQUE="\
$(cat /sys/devices/virtual/dmi/id/product_name)
$(cat /sys/class/dmi/id/board_version || echo "")
$(lscpu | grep "Model name" | head -n 1 | sed -r 's/Model name:\s{1,}(.*) @ .*z\s*/\1/g')
$(free | grep Mem: | awk '{print $2}')
$(upower -d | grep vendor: | head -n 1 | awk '{print $2}')
$(upower -d | grep model: | head -n 1 | awk '{print $2}')
$(upower -d | grep serial: | head -n 1 | awk '{print $2}')\
"
echo $UNIQUE | sha1sum | awk '{print $1}'
