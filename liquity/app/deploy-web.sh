#!/usr/bin/env bash

echo "Installing 'nvm'..."
wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")" & [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
nvm install 14
nvm use 14
echo "Starting to deploy 'web', installing..."
yarn install

echo "Prestarting 'web'..."
yarn prestart
echo "Building 'web'..."
yarn build
echo "#done"
