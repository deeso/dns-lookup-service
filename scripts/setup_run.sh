#!/bin/bash
MASTER_CONFIG=../examples/sample-config.toml
MASTER_LOG_CONFIG=../examples/sample-log4rs.toml

if [ -n "$1" ]
  then
    MASTER_CONFIG=$1
fi

if [ -n "$2" ]
  then
    MASTER_LOG_CONFIG=$2
fi

# create configs
cp $MASTER_CONFIG ../configs/config.toml
cp $MASTER_LOG_CONFIG ../configs/log_config.toml

# create dockers
cd ../docker/
chmod +x setup_everything.sh
./setup_everything.sh
cd ../scripts

# done
