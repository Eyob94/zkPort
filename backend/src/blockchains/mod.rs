#![allow(unused)]
use async_trait::async_trait;
use std::fmt::Debug;

use crate::utils::RetryPolicy;

mod solana;

#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// Symbol, Eg: SOL for Solana
    pub symbol: String,
    /// Amount in that account
    pub amount: u64,
    /// Public Address of that account
    pub address: [u8; 32],
}

impl AccountInfo {
    pub fn new(symbol: impl AsRef<str>, amount: u64, address: [u8; 32]) -> anyhow::Result<Self> {
        let symbol = symbol.as_ref();
        if symbol.is_empty() {
            anyhow::bail!("Symbol cannot be empty");
        }
        if address.is_empty() {
            anyhow::bail!("Address cannot be empty");
        }

        Ok(Self {
            symbol: symbol.into(),
            amount,
            address,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Localnet,
    Devnet,
    Mainnet,
}

#[derive(Debug, Clone)]
pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub environment: Environment,
    pub rpc_url: Option<String>,
    pub timeout_milliseconds: u64,
    pub commitment_level: Commitment,
    pub retry_policy: RetryPolicy,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Localnet,
            rpc_url: None,
            timeout_milliseconds: 5000,
            commitment_level: Commitment::Confirmed,
            retry_policy: RetryPolicy::default(),
        }
    }
}

impl ClientConfig {
    pub fn new_with_rpc(environment: Environment, rpc_url: String) -> Self {
        Self {
            environment,
            rpc_url: Some(rpc_url),
            ..Default::default()
        }
    }

    pub fn with_commitment(mut self, commitment: Commitment) -> Self {
        self.commitment_level = commitment;
        self
    }

    pub fn with_retry_policy(mut self, retry_policy: RetryPolicy) -> Self {
        self.retry_policy = retry_policy;
        self
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_milliseconds = timeout_ms;
        self
    }
}

#[async_trait]
pub trait BlockchainClient: Send + Sync + Debug {
    /// Initialize the client to interact with the respective blockchain
    async fn new(config: ClientConfig) -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Get Info about account from the respective blockchain
    async fn get_account_info(&self, pubkey: [u8; 32]) -> anyhow::Result<AccountInfo>;

    /// Get multiple accounts in a batch
    // async fn get_multiple_accounts(&self, addresses: &[String])
    //     -> anyhow::Result<Vec<AccountInfo>>;

    /// Health check to ensure connectivity
    async fn health_check(&self) -> anyhow::Result<()>;

    /// Get the current network/chain ID
    async fn get_chain_id(&self) -> anyhow::Result<String>;

    /// Get token balance,
    async fn get_token_balance(&self, address: &str, token_symbol: &str) -> anyhow::Result<u64>;

    /// Reconnect if the connection is lost
    async fn reconnect(&self) -> anyhow::Result<()> {
        Ok(()) // Default implementation does nothing
    }
}
