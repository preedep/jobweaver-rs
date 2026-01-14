#docker-compose up -d
docker run -d -p 9990:8080 -v $(pwd)/output:/app/data jobweaver-web
