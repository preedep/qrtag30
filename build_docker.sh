#docker rmi -f $(docker images -f “dangling=true” -q)

docker rmi -f prompt_pay_service:latest
docker build -t prompt_pay_service .
