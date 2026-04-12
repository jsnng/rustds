
terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "=4.64.0"
    }
  }
}

provider "azurerm" {
  features {
    resource_group {
      prevent_deletion_if_contains_resources = true
    }
  }
}
variable "resource_group_name" {
  type      = string
  sensitive = true
}
resource "azurerm_resource_group" "main" {
  name     = vars.resource_group_name
  location = "australiaeast"
}

variable "password" {
  type      = string
  sensitive = true
}
variable "azuread_login" {
  type      = string
  sensitive = true
}
variable "azuread_object_id" {
  type      = string
  sensitive = true
}

variable "azure_mssql_server_name" {
  type      = string
  sensitive = true
}

resource "azurerm_mssql_server" "main" {
  name                         = vars.azure_mssql_server_name
  resource_group_name          = azurerm_resource_group.main.name
  location                     = azurerm_resource_group.main.location
  version                      = "12.0"
  administrator_login          = "gemagroup"
  administrator_login_password = var.password
  azuread_administrator {
    login_username = var.azuread_login
    object_id      = var.azuread_object_id
  }
}

variable "azurerm_mssql_database_name" {
  type      = string
  sensitive = true
}

resource "azurerm_mssql_database" "main" {
  name      = var.azurerm_mssql_database_name
  server_id = azurerm_mssql_server.main.id
  sku_name  = "Basic"
}
