#!/bin/bash

SOFTWARE=$1
VERSION=${2:-latest}

case $SOFTWARE in
    "misskey")
        export SOFTWARE_NAME=misskey
        export CONTAINER_IMAGE=misskey/misskey
        ;;
    "cherrypick")
        export SOFTWARE_NAME=cherrypick
        export CONTAINER_IMAGE=noridev/cherrypick
        ;;
    "sharkey")
        export SOFTWARE_NAME=sharkey
        export CONTAINER_IMAGE=registry.activitypub.software/transfem-org/sharkey
        ;;
    *)
        echo "Unsupported software: $SOFTWARE. If you want to add it, please create an issue or a pull request."
        exit 1
        ;;
esac

export SOFTWARE_VERSION=$VERSION

cat > .env.software << EOF
SOFTWARE_NAME=$SOFTWARE_NAME
SOFTWARE_VERSION=$SOFTWARE_VERSION
CONTAINER_IMAGE=$CONTAINER_IMAGE
EOF
