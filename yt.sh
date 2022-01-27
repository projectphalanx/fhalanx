#!/bin/sh
trap "kill 0" EXIT
europa --tmp --dev > ~/foo.out 2> ~/foo.err &
clear
yarn test


