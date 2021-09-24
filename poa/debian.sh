#!/bin/bash

#echo "Put the binary"
#install -Dm 755 "target/release/$2" -t "$2/usr/bin"
#if [[ $? -ne 0 ]]; then
  #echo "Oops... A error"
  #exit 1
#fi

echo "Creating iiabc tree..."
mkdir -p apmpkg/etc/apmpkg/iiabc
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi

echo "Coping iiabc..."
cp -r src/iiabc/ apmpkg/etc/apmpkg/
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi

echo "Make paquetes folder.."
mkdir -p apmpkg/etc/apmpkg/paquetes
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi
