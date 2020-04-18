#!/usr/bin/env bash

set -e

node_check=$(echo $(node -v) grep -q 'v' || echo '9042')
if [[ $node_check == '9042' ]]
then
  echo 'nodejs not on this machine - exiting..' && exit 1
fi

if [[ -d artifact ]]
then
  rm -rf artifact
fi

git clone https://github.com/selfup/hyperapp-one artifact

cd artifact && npm i && cd -

./scripts/run.release.sh
