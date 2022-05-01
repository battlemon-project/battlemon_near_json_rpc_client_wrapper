use near_crypto::{InMemorySigner, SecretKey};
pub use near_jsonrpc_client::NEAR_MAINNET_ARCHIVAL_RPC_URL;
pub use near_jsonrpc_client::NEAR_TESTNET_ARCHIVAL_RPC_URL;
use near_jsonrpc_client::{methods, AsUrl, JsonRpcClient};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockId, BlockReference, Finality};

type GenericError = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, GenericError>;

pub struct JsonRpcWrapper {
    client: JsonRpcClient,
    signer: InMemorySigner,
}

impl JsonRpcWrapper {
    pub fn connect<T: AsUrl>(
        server_address: T,
        signer_account_id: AccountId,
        signer_secret_key: SecretKey,
    ) -> Self {
        Self {
            client: JsonRpcClient::connect(server_address),
            signer: InMemorySigner::from_secret_key(signer_account_id, signer_secret_key),
        }
    }

    pub async fn block_height_from_hash(&self, block_hash: CryptoHash) -> Result<u64> {
        let request = methods::block::RpcBlockRequest {
            block_reference: BlockReference::BlockId(BlockId::Hash(block_hash)),
        };
        let response = self.client.call(request).await?;
        Ok(response.header.height)
    }

    pub async fn final_block_height(&self) -> Result<u64> {
        let request = methods::block::RpcBlockRequest {
            block_reference: BlockReference::Finality(Finality::Final),
        };

        let response = self.client.call(request).await?;

        Ok(response.header.height)
    }
}
