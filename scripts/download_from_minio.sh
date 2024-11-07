#!/usr/bin/env sh

# Script to download a file from MinIO
# Usage: ./download_from_minio.sh <url> <username> <password> <bucket> <file_path_in_minio> <destination_path>
# Example: ./download_from_minio.sh "localhost:9000" $MINIO_ROOT_USER $MINIO_ROOT_PASSWORD kms PUB-p1/CRS/7f0979b779f29c6e94921f5536e2434ac6bd1596 $PWD/CRS

# Function to print an error message and exit
print_error_and_exit() {
  echo "$1"
  exit 1
}

# Check if all required arguments are provided
[ -z "$1" ] && print_error_and_exit "You have NOT specified a MINIO URL!"
[ -z "$2" ] && print_error_and_exit "You have NOT specified a USERNAME!"
[ -z "$3" ] && print_error_and_exit "You have NOT specified a PASSWORD!"
[ -z "$4" ] && print_error_and_exit "You have NOT specified a BUCKET!"
[ -z "$5" ] && print_error_and_exit "You have NOT specified a FILE PATH in MinIO!"
[ -z "$6" ] && print_error_and_exit "You have NOT specified a DESTINATION PATH!"

# User MinIO Vars
URL=$1
USERNAME=$2
PASSWORD=$3
BUCKET=$4
FILE_PATH=$5
DESTINATION_PATH=$6

# Static Vars
DATE=$(date -R --utc)
OBJ_PATH="/${BUCKET}/${FILE_PATH}"
CONTENT_TYPE="application/octet-stream"
SIG_STRING="GET\n\n\n${DATE}\n${OBJ_PATH}"
SIGNATURE=$(echo -en "${SIG_STRING}" | openssl sha1 -hmac "${PASSWORD}" -binary | base64)

# Download the file using curl with better error handling
curl --silent --fail -X GET \
    -H "Host: $URL" \
    -H "Date: ${DATE}" \
    -H "Authorization: AWS ${USERNAME}:${SIGNATURE}" \
    http://$URL${OBJ_PATH} -o ${DESTINATION_PATH}

# Check if the download was successful
if [ $? -eq 0 ]; then
  FILE_SIZE=$(stat -c%s "$DESTINATION_PATH")
  HUMAN_READABLE_SIZE=$(numfmt --to=iec --suffix=B "$FILE_SIZE")
  echo "File downloaded successfully to ${DESTINATION_PATH} (Size: ${HUMAN_READABLE_SIZE})"
else
  print_error_and_exit "File download failed! Please check your inputs and try again."
fi
