#! /bin/zsh

cat > terraform.tfvars <<EOF
$(cat template.tfvars)
azuread_object_id = "$(az ad signed-in-user show --query id -o tsv)"
azuread_login     = "$(az ad signed-in-user show --query userPrincipalName -o tsv)"
