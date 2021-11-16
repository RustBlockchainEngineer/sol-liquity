#!/usr/bin/env bash
echo "Starting to deploy 'web', installing..."
yarn install
echo "Prestarting 'web'..."
yarn prestart
echo "Building 'web'..."
yarn build
echo "#done"
