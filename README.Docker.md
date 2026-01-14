# Docker Deployment Guide

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Build and start the container
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the container
docker-compose down
```

The application will be available at `http://localhost:8080`

### Using Docker directly

```bash
# Build the image
docker build -t jobweaver-web .

# Run the container
docker run -d \
  --name jobweaver \
  -p 8080:8080 \
  -v $(pwd)/data:/app/data \
  jobweaver-web

# View logs
docker logs -f jobweaver

# Stop the container
docker stop jobweaver
docker rm jobweaver
```

## Image Details

- **Base OS**: Alpine Linux 3.19 (~5MB)
- **Final Image Size**: ~30-40MB (including application)
- **Architecture**: Multi-stage build for minimal size
- **Security**: Runs as non-root user (jobweaver:1000)

## Volume Mounts

- `/app/data` - Database directory (mount your database here)

## Environment Variables

- `RUST_LOG` - Log level (default: `info`)
- `DATABASE_PATH` - Path to SQLite database (default: `/app/data/controlm.db`)

## Health Check

The container includes a health check that verifies the application is responding:
- Interval: 30 seconds
- Timeout: 3 seconds
- Retries: 3

## Building for Production

```bash
# Build with specific tag
docker build -t jobweaver-web:1.0.0 .

# Push to registry
docker tag jobweaver-web:1.0.0 your-registry/jobweaver-web:1.0.0
docker push your-registry/jobweaver-web:1.0.0
```

## Troubleshooting

### Check container status
```bash
docker ps -a
```

### View logs
```bash
docker logs jobweaver
```

### Execute shell in container
```bash
docker exec -it jobweaver sh
```

### Check health status
```bash
docker inspect --format='{{.State.Health.Status}}' jobweaver
```
