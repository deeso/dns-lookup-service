DOCKER_TAG=rust:latest
DOCKER_NAME=dns-lookup-service

RUST_BIN_NAME='rust-bin'
CARGO_DO_RELEASE= # "--release"
SERVICE=21021

# base how to set-up docker base on GIT_REPO variable
TMP_DIR=tmp-git
GIT_REPO=https://github.com/deeso/dns-lookup-service.git
BASE_DIR=../../
RUST_SRC=$BASE_DIR

# cleaup Docker
docker kill $DOCKER_NAME
docker rm $DOCKER_NAME
rm -fr config.toml log_config.toml dns-lookup-service

if [ ! -z "$GIT_REPO" ] 
then
    git clone $GIT_REPO $TMP_DIR
    RUST_SRC=$TMP_DIR

fi

CARGO_BUILD_ARGS=
DEBUG_BIN=$RUST_SRC/target/debug/$DOCKER_NAME
RELEASE_BIN=$RUST_SRC/target/debug/$DOCKER_NAME
TARGET_BIN=$DEBUG_BIN

CONFIGS_DIR=$BASE_DIR/configs/


CONF_FILE=$CONFIGS_DIR/config.toml
LOG_CONF_FILE=$CONFIGS_DIR/log_config.toml

MAIN= #$MAINS_DIR/run-all-multiprocess.py
HOST_FILE= #$CONFIGS_DIR/hosts

DOCKER_ADD_HOST=
if [ ! -z "$HOST_FILE" ] 
then
    MONGODB_HOST=$(cat $HOST_FILE | grep "mongodb-host")
    REDIS_QUEUE_HOST=$(cat $HOST_FILE | grep "redis-queue-host")
    DOCKER_ADD_HOST=" --add-host $MONGODB_HOST --add-host $REDIS_QUEUE_HOST "    
fi

cp $CONF_FILE config.toml
cp $LOG_CONF_FILE log_config.toml
# hack

BACK=`pwd`
cd $RUST_SRC
if [ ! -z "$CARGO_DO_RELEASE" ] 
then
    cargo build --release $CARGO_BUILD_ARGS
    TARGET_BIN=$RELEASE_BIN
else
    cargo build $CARGO_BUILD_ARGS
fi

cd $BACK
cp $TARGET_BIN $RUST_BIN_NAME

# setup dirs
DOCKER_BASE=/data
DOCKER_NB=$DOCKER_BASE/$DOCKER_NAME
DOCKER_LOGS=$DOCKER_NB/logs
DOCKER_DATA=$DOCKER_NB/data

DOCKER_PORTS="-p $SERVICE:$SERVICE"
DOCKER_ENV=""
DOCKER_VOL=""

mkdir -p $DOCKER_DATA
mkdir -p $DOCKER_LOGS
chmod -R a+rw $DOCKER_NB

# TODO comment below if you want to save to Mongo
echo "./$RUST_BIN_NAME --config config.toml --log_config log_config.toml --iron_server" > rust_cmd.sh
cat rust_cmd.sh

#docker build --no-cache -t $DOCKER_TAG .
docker build -t $DOCKER_TAG .

# clean up here
rm -fr config.toml log_config.toml rust_cmd.sh $RUST_BIN_NAME $TMP_DIR
echo "rm -fr config.toml rust_cmd.sh $RUST_BIN_NAME $TMP_DIR"

# run command not 
echo "docker run $DOCKER_PORTS $DOCKER_VOL -it $DOCKER_ENV \
           --name $DOCKER_NAME $DOCKER_TAG"

docker run -d $DOCKER_ADD_HOST $DOCKER_PORTS $DOCKER_VOL -it $DOCKER_ENV \
            --name $DOCKER_NAME $DOCKER_TAG
