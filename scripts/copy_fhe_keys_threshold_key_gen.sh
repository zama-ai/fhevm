#!/usr/bin/env bash

# This bash script creates global fhe keys
# and copy them to the the given folder
# It accepts
# - the volume name from which we want to copy keys
# - the LOCAL_BUILD_PUBLIC_KEY_PATH as the second optional parameter.

set -Eeuo pipefail

if [ "$#" -lt 1 ]; then
    echo "Usage: $(basename "$0") zama-kms-threshold_keys [LOCAL_BUILD_PUBLIC_KEY_PATH] "
    echo "Example: $(basename "$0") zama-kms-threshold_keys $(PWD)/network-fhe-keys "
    exit
fi


VOLUME_NAME=$1
echo "$VOLUME_NAME"
CURRENT_FOLDER=$PWD


KEYS_FULL_PATH=$CURRENT_FOLDER/res/keys
mkdir -p $KEYS_FULL_PATH

if [ "$#" -ge 2 ]; then
    LOCAL_BUILD_PUBLIC_KEY_PATH=$2
    
    NETWORK_KEYS_PUBLIC_PATH="${LOCAL_BUILD_PUBLIC_KEY_PATH}"
else
    NETWORK_KEYS_PUBLIC_PATH="./volumes/network-public-fhe-keys"
fi

mkdir -p "$KEYS_FULL_PATH"



# Check if the volume exists
if docker volume inspect "$VOLUME_NAME" >/dev/null 2>&1; then
    echo "Volume '$VOLUME_NAME' exists. Proceeding with the copy..."

    # Step 1: Start a temporary container with the volume attached
    docker run --rm -d --name temp_container -v "$VOLUME_NAME":/volume_data alpine tail -f /dev/null

    # Step 2: Copy the contents of the volume to the local directory
    docker cp temp_container:/volume_data/. "$KEYS_FULL_PATH"
    echo "Contents of '$VOLUME_NAME' copied to '$KEYS_FULL_PATH'."

    # Step 3: Stop and remove the temporary container
    docker stop temp_container
    echo "Temporary container stopped and removed."

else
    echo "Volume '$VOLUME_NAME' does not exist. Exiting."
fi


echo "$KEYS_FULL_PATH"



echo "###########################################################"
echo "Keys creation is done, they are stored in $KEYS_FULL_PATH"
echo "###########################################################"

echo "$NETWORK_KEYS_PUBLIC_PATH"

PKS="PUB-p1/PublicKey/d4d17a412a6533599b010c8ffc3d6ebdc6b1cfad"
SKS="PUB-p1/ServerKey/d4d17a412a6533599b010c8ffc3d6ebdc6b1cfad"
CRS="PUB-p1/CRS/d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906"
SIGNER1="PUB-p1/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"
SIGNER2="PUB-p2/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"
SIGNER3="PUB-p3/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"
SIGNER4="PUB-p4/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"

MANDATORY_KEYS_LIST=($PKS $SKS $SIGNER1 $SIGNER2 $SIGNER3 $SIGNER4 $CRS)

for key in "${MANDATORY_KEYS_LIST[@]}"; do
    if [ ! -f "$KEYS_FULL_PATH/$key" ]; then
        echo "#####ATTENTION######"
        echo "$key does not exist in $KEYS_FULL_PATH!"
        echo "####################"
        exit
    fi
    
done


echo "###########################################################"
echo "All the required keys exist in $KEYS_FULL_PATH"
echo "###########################################################"

mkdir -p $NETWORK_KEYS_PUBLIC_PATH


echo "Copying $SKS to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
cp $KEYS_FULL_PATH/$SKS $NETWORK_KEYS_PUBLIC_PATH/sks

echo "Copying $PKS to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
cp $KEYS_FULL_PATH/$PKS $NETWORK_KEYS_PUBLIC_PATH/pks

echo "Copying $CRS to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
cp $KEYS_FULL_PATH/$CRS $NETWORK_KEYS_PUBLIC_PATH/pp

echo "Copying $SIGNER1 to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
cp $KEYS_FULL_PATH/$SIGNER1 $NETWORK_KEYS_PUBLIC_PATH/signer1

echo "Copying $SIGNER2 to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
cp $KEYS_FULL_PATH/$SIGNER2 $NETWORK_KEYS_PUBLIC_PATH/signer2

echo "Copying $SIGNER3 to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
cp $KEYS_FULL_PATH/$SIGNER3 $NETWORK_KEYS_PUBLIC_PATH/signer3

echo "Copying $SIGNER4 to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
cp $KEYS_FULL_PATH/$SIGNER4 $NETWORK_KEYS_PUBLIC_PATH/signer4
