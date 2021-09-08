#!/bin/sh
cat $1 | grep ^version | awk '{print $3}' | tr -d '"'
