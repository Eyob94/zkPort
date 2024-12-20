use std::fmt::Debug;
use std::time::Duration;

use anyhow::bail;
use async_trait::async_trait;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use super::{AccountInfo, BlockchainClient, ClientConfig, Commitment, Environment};

struct SolanaClient {
    client: RpcClient,
    config: ClientConfig,
}

impl Debug for SolanaClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolanaClient")
            .field("config", &self.config)
            .field("rpc_client", &"RpcClient")
            .finish()
    }
}

impl From<Commitment> for CommitmentConfig {
    fn from(value: Commitment) -> Self {
        match value {
            Commitment::Processed => CommitmentConfig::processed(),
            Commitment::Confirmed => CommitmentConfig::confirmed(),
            Commitment::Finalized => CommitmentConfig::finalized(),
        }
    }
}

#[async_trait]
impl BlockchainClient for SolanaClient {
    async fn new(config: ClientConfig) -> anyhow::Result<Self> {
        if let Some(rpc_url) = &config.rpc_url {
            let client = RpcClient::new_with_timeout_and_commitment(
                rpc_url.clone(),
                Duration::from_millis(config.timeout_milliseconds),
                config.commitment_level.clone().into(),
            );

            return Ok(Self { client, config });
        }
        bail!("No rpc_url found")
    }

    async fn get_account_info(&self, pubkey: [u8; 32]) -> anyhow::Result<AccountInfo> {
        let pubkey = Pubkey::from(pubkey);

        let account = self.client.get_account(&pubkey).await?;

        Ok(AccountInfo::new(
            "SOL",
            account.lamports,
            account.owner.to_bytes(),
        )?)
    }

    async fn health_check(&self) -> anyhow::Result<()> {
        self.client.get_health().await?;

        Ok(())
    }

    async fn get_chain_id(&self) -> anyhow::Result<String> {
        let genesis_hash = self.client.get_genesis_hash().await?;

        let network = match self.config.environment {
            Environment::Mainnet => "mainnet-beta",
            Environment::Devnet => "devnet",
            Environment::Localnet => "localnet",
        };

        Ok(format!("solana-{}-{}", network, genesis_hash))
    }

    async fn get_token_balance(&self, address: &str, token_symbol: &str) -> anyhow::Result<u64> {
        todo!()
    }
}
