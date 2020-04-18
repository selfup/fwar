# fwar

_alpha stage_

File System War. Scan source code files and find bad actors. Memory safety is paramount.

#### use

Assumes a directory called `artifact` is in the current directory. Scans said directory.

#### testing

`./scripts/test.sh`

1. this will pull down [hyperapp-one](https://github.com/selfup/hyperapp-one)
1. run `npm install`
1. then scan all ~50k files (source code and node_modules)
