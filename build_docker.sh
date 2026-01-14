#!/bin/bash

# Build Docker image with multi-platform support
# This script builds for both amd64 (Intel/AMD) and arm64 (Apple Silicon) architectures

set -e

IMAGE_NAME="jobweaver-web"

echo "🔨 Building multi-platform Docker image..."
echo "   Platforms: linux/amd64, linux/arm64"
echo ""

# Create and use buildx builder if not exists
if ! docker buildx inspect multiplatform-builder > /dev/null 2>&1; then
    echo "📦 Creating buildx builder..."
    docker buildx create --name multiplatform-builder --use
else
    echo "📦 Using existing buildx builder..."
    docker buildx use multiplatform-builder
fi

# Build for multiple platforms
echo "🏗️  Building image..."
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    -t "$IMAGE_NAME" \
    --load \
    .

echo ""
echo "✅ Build completed successfully!"
echo "   Image: $IMAGE_NAME"
echo "   Platforms: linux/amd64, linux/arm64"
