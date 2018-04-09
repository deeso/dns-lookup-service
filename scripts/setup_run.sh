#!/bin/bash
MASTER_CONFIG=../examples/sample-config.toml

if [ -n "$1" ]
  then
    MASTER_CONFIG=$1
fi

# create configs
cp $MASTER_CONFIG ../configs/config.toml

# create dockers
cd ../docker/
chmod +x setup_everything.sh
./setup_everything.sh
cd ../scripts

# done
