#! /bin/zsh
podman machine init
podman machine start
podman run -e "ACCEPT_EULA=1" -e 'MSSQL_SA_PASSWORD=Passw0rd!' -e "MSSQL_PID=Developer" -p 1433:1433 mcr.microsoft.com/azure-sql-edge