#!/bin/bash

#configuration
APP_NAME="Hello World"
APP_DESC="Luna's very useful program"
APP_AUTHOR="Luna"

APP_ICON="./icon.png"
APP_BANNER="./banner.png" #This program assumes a 2d banner, for a 3d banner change -i in the makebanner command to -ci
APP_BANNER_SOUND="./audio.cwav"

LINK_IP_ADDR="192.168.0.18"

ELF_NAME="Hello-World"

make_3dsx(){
  echo "make_3dsx"
  3dsxtool "./target/armv6k-nintendo-3ds/debug/$ELF_NAME.elf" "./build/$APP_NAME.3dsx" --smdh="./build/$APP_NAME.smdh"
}

make_cia(){
  echo "make_cia"
  makerom -f cia -o "./build/$APP_NAME.cia" -target t -elf "./target/armv6k-nintendo-3ds/debug/$ELF_NAME.elf" -banner "./build/banner.bin" -rsf ./app.rsf -icon "./build/Hello World.smdh"
}

run_3dslink(){
  echo "run_3dslink"
  make_3dsx
  3dslink -a "$LINK_IP_ADDR" "./build/$APP_NAME.3dsx"
}

#make smdh
echo "making smdh"
bannertool makebanner -i "$APP_BANNER" -ca "$APP_BANNER_SOUND" -o "./build/banner.bin"
bannertool makesmdh -s "$APP_NAME" -l "$APP_DESC" -p "$APP_AUTHOR" -i "$APP_ICON" -o "./build/$APP_NAME.smdh"

echo "cargo build"
cargo 3ds build #TODO: get this thing to only make the elf and not an smdh or 3dsx

for i in "$@"; do
  case $i in
    build) make_cia && make_3dsx ;;
    3dslink) run_3dslink ;;
  esac
done
