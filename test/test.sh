#!/usr/bin/env bash

BASEDIR=$(dirname "$0")
TEST_DIR="$(realpath "${BASEDIR}")"
PROJCET_DIR="$(realpath "${TEST_DIR}/..")"

cd $PROJCET_DIR

cargo r test/1.txt
