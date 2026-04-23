#! /bin/zsh

ASSWORD=$(grep -o '"password":"[^"]*"' terraform.tfstate | cut -d'"' -f4)
MY_IP=$(curl -s ifconfig.me)

az sql server firewall-rule create \
  --resource-group $RG \
  --server $SERVER \
  --name my-ip \
  --start-ip-address "$MY_IP" \
  --end-ip-address "$MY_IP" \
  && { SSLKEYLOGFILE=/tmp/sskey.log cargo run --example cli --no-default-features --features "rustls,tds7.4,std,tls1.2" -- \
    -S $SERVER.database.windows.net \
    -P 1433 \
    -U $USER \
    -p "$PASSWORD" \
    -d test
  }
  ; az sql server firewall-rule delete \
    --resource-group $RG \
    --server $SERVER \
    --name my-ip