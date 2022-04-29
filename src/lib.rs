use near_jsonrpc_client::{methods, AsUrl, JsonRpcClient};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{BlockId, BlockReference};

type GenericError = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, GenericError>;

pub struct JsonRpcWrapper(JsonRpcClient);

impl JsonRpcWrapper {
    pub fn connect<T: AsUrl>(server_address: T) -> Self {
        Self(JsonRpcClient::connect(server_address))
    }

    pub async fn block_height_from_hash(&self, block_hash: CryptoHash) -> Result<u64> {
        let request = methods::block::RpcBlockRequest {
            block_reference: BlockReference::BlockId(BlockId::Hash(block_hash)),
        };
        let response = self.0.call(request).await?;
        Ok(response.header.height)
    }
}