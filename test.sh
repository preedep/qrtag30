curl -v --header "Content-Type: application/json" \
  --request POST \
  --data '{"merchant_name":"test","mobile_number":"0809729900","transaction_amount":20}' \
  http://localhost:8080/promptpay/qrcode






