#!/bin/bash

echo "Put the binary"
install -Dm 755 "target/release/apmpkg" -t "apmpkg/usr/bin"
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi

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

echo "Put manpage (español)"
install -Dm 644 "man/apmpkg.1" -t apmpkg/usr/share/man/man1
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi

echo "Put manpage (english)"
install -Dm 644 "man/apmpkg-en.1" -t apmpkg/usr/share/man/man1
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi

echo "Installing bash completions"
install -Dm 644 "completions/apmpkg.bash-completion" -t apmpkg/usr/share/bash-completion/bash_completion
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi

echo "Installing zsh completions"
install -Dm 644 "completions/_apmpkg" -t apmpkg/usr/share/bash-completion/completions/
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi

echo "Installing fish completions"
install -Dm 644 "completions/apmpkg.fish" -t apmpkg/usr/share/fish/vendor_completions.d/
if [[ $? -ne 0 ]]; then
  echo "Oops... A error"
  exit 1
fi
