read -p "Press any key to continue..." -n1 -s

docker image tag prompt_pay_service:latest eaacrglobal101.azurecr.io/rust-prompt-pay-service:latest
az login

read -p "Press any key to continue... " -n1 -s

az acr login --name eaacrglobal101
docker push eaacrglobal101.azurecr.io/rust-prompt-pay-service:latest

