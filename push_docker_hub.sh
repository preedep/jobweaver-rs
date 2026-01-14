#!/bin/bash

# Docker Hub push script for jobweaver-web with multi-platform support
# Usage: ./push_docker_hub.sh [version]
# Example: ./push_docker_hub.sh 1.0.0

set -e

# Configuration
DOCKER_USERNAME="${DOCKER_USERNAME:-nickmsft}"
IMAGE_NAME="jobweaver-web"

# Get version from argument or use 'latest'
VERSION="${1:-latest}"

echo "🚀 Building and pushing multi-platform Docker image to Docker Hub..."
echo "   Username: $DOCKER_USERNAME"
echo "   Image: $IMAGE_NAME"
echo "   Version: $VERSION"
echo "   Platforms: linux/amd64, linux/arm64"
echo ""

# Check if logged in to Docker Hub
if ! docker info | grep -q "Username: $DOCKER_USERNAME" 2>/dev/null; then
    echo "⚠️  Not logged in to Docker Hub"
    echo "   Running: docker login"
    docker login
fi

# Create and use buildx builder if not exists
if ! docker buildx inspect multiplatform-builder > /dev/null 2>&1; then
    echo "📦 Creating buildx builder..."
    docker buildx create --name multiplatform-builder --use
else
    echo "📦 Using existing buildx builder..."
    docker buildx use multiplatform-builder
fi

# Build and push for multiple platforms
echo "🏗️  Building and pushing multi-platform image..."

# Build tags
TAGS="-t $DOCKER_USERNAME/$IMAGE_NAME:$VERSION"
if [ "$VERSION" != "latest" ]; then
    TAGS="$TAGS -t $DOCKER_USERNAME/$IMAGE_NAME:latest"
fi

# Build and push
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    $TAGS \
    --push \
    .

echo ""
echo "✅ Successfully pushed to Docker Hub!"
echo "   Image: $DOCKER_USERNAME/$IMAGE_NAME:$VERSION"
if [ "$VERSION" != "latest" ]; then
    echo "   Also tagged as: $DOCKER_USERNAME/$IMAGE_NAME:latest"
fi
echo "   Platforms: linux/amd64, linux/arm64"
echo ""
echo "📋 To pull this image:"
echo "   docker pull $DOCKER_USERNAME/$IMAGE_NAME:$VERSION"
echo ""
echo "💡 The image will work on:"
echo "   - Intel/AMD servers (linux/amd64)"
echo "   - Apple Silicon Macs (linux/arm64)"
