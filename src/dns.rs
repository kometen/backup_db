pub mod dns {

    use anyhow::{Context, Result};
    use azure_vault_secrets::Vault;
    use hickory_resolver::error::ResolveError;
    use hickory_resolver::system_conf::read_system_conf;
    use hickory_resolver::AsyncResolver;

    use crate::DatabaseConfig;

    /// # Arguments
    ///
    /// * `vault` - Name of the variable with values from Azure Key Vault
    ///
    /// # Returns
    ///
    /// A Result containing the resolved hostname + domainname if successful,
    /// or an error if the hostname + domainname could not be resolved.
    ///
    pub async fn check_dns(vault: &Vault) -> Result<()> {
        let (config, opts) = read_system_conf().map_err(|e| ResolveError::from(e))?;
        let resolver = AsyncResolver::tokio(config, opts);
        let config = DatabaseConfig::from_vault(vault)?;

        let hostname = format!("{}.{}.", &config.host(), &config.domain());

        let response = resolver
            .lookup_ip(&hostname)
            .await
            .with_context(|| format!("Failed to resolve hostname: {}", hostname))?;

        if response.iter().next().is_none() {
            anyhow::bail!("No IP address found for hostname: {}", hostname);
        }

        Ok(())
    }
}
