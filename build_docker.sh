#docker rmi -f $(docker images -f “dangling=true” -q)

docker rmi -f prompt_pay_service:latest
docker build --platform linux/amd64 -t prompt_pay_service .
