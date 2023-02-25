#!/usr/bin/env sh
set -e

pushd .

# The following lines ensure we run from the project root
DOCKER_DIR=$(dirname "$(readlink -f "$0")")
PROJECT_ROOT=$(dirname "$DOCKER_DIR")

cd "$PROJECT_ROOT"

DOCKER_ORG=cybros-network
DOCKER_REPO=node
DOCKER_TAG=latest

# Build the image
echo "Building ${DOCKER_ORG}/${DOCKER_REPO}:${DOCKER_TAG} docker image, hang on!"
time DOCKER_BUILDKIT=1 docker build -f ./docker/node.Dockerfile -t ${DOCKER_ORG}/${DOCKER_REPO}:${DOCKER_TAG} .

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${DOCKER_REPO}

popd
