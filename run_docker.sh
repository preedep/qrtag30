docker stop prompt_pay_service
docker rm prompt_pay_service

docker run --name prompt_pay_service  -v $(pwd)/tmp:/tmp/qrcode -p 8080:8080  -d prompt_pay_service:latest
