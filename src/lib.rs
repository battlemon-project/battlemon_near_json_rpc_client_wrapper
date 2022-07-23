use near_crypto::InMemorySigner;
pub use near_jsonrpc_client::NEAR_MAINNET_ARCHIVAL_RPC_URL;
pub use near_jsonrpc_client::NEAR_TESTNET_ARCHIVAL_RPC_URL;
use near_jsonrpc_client::{methods, AsUrl, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::Transaction;
pub use near_primitives::types::AccountId;
use near_primitives::types::{Balance, BlockId, BlockReference, Finality, Gas};
use std::fmt;

pub struct JsonRpcWrapper {
    client: JsonRpcClient,
    signer: InMemorySigner,
}

impl JsonRpcWrapper {
    pub fn connect<T: AsUrl>(server_address: T, signer: InMemorySigner) -> Self {
        Self {
            client: JsonRpcClient::connect(server_address),
            signer,
        }
    }

    pub async fn block_height_from_hash(&self, block_hash: CryptoHash) -> anyhow::Result<u64> {
        let request = methods::block::RpcBlockRequest {
            block_reference: BlockReference::BlockId(BlockId::Hash(block_hash)),
        };
        let response = self.client.call(request).await?;
        Ok(response.header.height)
    }

    pub async fn final_block_height(&self) -> anyhow::Result<u64> {
        let request = methods::block::RpcBlockRequest {
            block_reference: BlockReference::Finality(Finality::Final),
        };

        let response = self.client.call(request).await?;

        Ok(response.header.height)
    }

    pub async fn call_contract_method<T: serde::Serialize + fmt::Display>(
        &self,
        contract_id: AccountId,
        method: String,
        json_args: T,
        gas: Gas,
        deposit: Balance,
    ) -> anyhow::Result<()> {
        let access_key_query_response = self
            .client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request: near_primitives::views::QueryRequest::ViewAccessKey {
                    account_id: self.signer.account_id.clone(),
                    public_key: self.signer.public_key.clone(),
                },
            })
            .await?;

        let current_nonce = match access_key_query_response.kind {
            QueryResponseKind::AccessKey(access_key) => access_key.nonce,
            _ => return Err(anyhow::anyhow!("Failed to extract current nonce")),
        };
        let args = json_args.to_string().into_bytes();
        let transaction = Transaction::new(
            self.signer.account_id.clone(),
            self.signer.public_key.clone(),
            contract_id,
            current_nonce + 1,
            access_key_query_response.block_hash,
        )
        .function_call(method, args, gas, deposit);

        let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction: transaction.sign(&self.signer),
        };

        self.client.call(request).await?;

        Ok(())
    }
}
