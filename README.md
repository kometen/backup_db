# backup_db

A wrapper around pg_dump so information about host to connect to, database, password is retrieved from Azure Key Vault
and URL to Azure Key Vault from 1password thus avoiding storing passwords in cleartext locally.

Requires 1password  and Azure command line utilities installed locally, an Azure-subscription, a PostgreSQL-client.
Log in with `az login`.

Some settings can be set in a `.env` file like this:

```
BUFFER_SIZE=16384
COMPRESSION_METHOD=lz4
FILE_PREFIX=production
FOLDER=backup
```

Add the following secrets to Azure Key Vault:

```
db-host
db-name
db-pwd
db-user
db-domain
```

Add an entry in 1password that can be accessed by the program. The path is formatted in Rust.

```
let op_path = format!("op://Production/AzureKeyVault{}/url", key);
```

An example of a path can be `op://Production/AzureKeyVaultInvoice/url`. The path can be changed to suit your own
requirement with the format `op://[vault-name]/[password-item]/[value]` in 1password.

Clone the repository, test, build and run with

```
cargo test
cargo build [--release]
./target/release/backup_db -n invoice
```

Install via brew tap.

```
brew tap kometen/backup-db
brew install backup-db
```

Build and run the container-image.

```
docker build -t backup_db:dev .
docker run --user backup_db_user backup_db:dev
```

The container will not inherit the environment from the shell so running from the container will not work.

Based on an example from https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/security_keyvault/examples/get_secret.rs
and guidance from both ChatGPT and Claude. Used the editor zed at `https://github.com/zed-industries/zed`.
