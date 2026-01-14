#!/bin/bash

# Docker Hub push script for jobweaver-web
# Usage: ./push_docker_hub.sh [version]
# Example: ./push_docker_hub.sh 1.0.0

set -e

# Configuration
DOCKER_USERNAME="${DOCKER_USERNAME:-nickmsft}"
IMAGE_NAME="jobweaver-web"
LOCAL_IMAGE="jobweaver-web"

# Get version from argument or use 'latest'
VERSION="${1:-latest}"

echo "🚀 Pushing Docker image to Docker Hub..."
echo "   Username: $DOCKER_USERNAME"
echo "   Image: $IMAGE_NAME"
echo "   Version: $VERSION"
echo ""

# Check if local image exists
if ! docker image inspect "$LOCAL_IMAGE" > /dev/null 2>&1; then
    echo "❌ Error: Local image '$LOCAL_IMAGE' not found!"
    echo "   Please run ./build_docker.sh first"
    exit 1
fi

# Check if logged in to Docker Hub
if ! docker info | grep -q "Username: $DOCKER_USERNAME"; then
    echo "⚠️  Not logged in to Docker Hub"
    echo "   Running: docker login"
    docker login
fi

# Tag the image
echo "🏷️  Tagging image..."
docker tag "$LOCAL_IMAGE" "$DOCKER_USERNAME/$IMAGE_NAME:$VERSION"

# Also tag as latest if version is specified
if [ "$VERSION" != "latest" ]; then
    docker tag "$LOCAL_IMAGE" "$DOCKER_USERNAME/$IMAGE_NAME:latest"
fi

# Push the versioned image
echo "📤 Pushing $DOCKER_USERNAME/$IMAGE_NAME:$VERSION..."
docker push "$DOCKER_USERNAME/$IMAGE_NAME:$VERSION"

# Push latest tag if version was specified
if [ "$VERSION" != "latest" ]; then
    echo "📤 Pushing $DOCKER_USERNAME/$IMAGE_NAME:latest..."
    docker push "$DOCKER_USERNAME/$IMAGE_NAME:latest"
fi

echo ""
echo "✅ Successfully pushed to Docker Hub!"
echo "   Image: $DOCKER_USERNAME/$IMAGE_NAME:$VERSION"
if [ "$VERSION" != "latest" ]; then
    echo "   Also tagged as: $DOCKER_USERNAME/$IMAGE_NAME:latest"
fi
echo ""
echo "📋 To pull this image:"
echo "   docker pull $DOCKER_USERNAME/$IMAGE_NAME:$VERSION"
