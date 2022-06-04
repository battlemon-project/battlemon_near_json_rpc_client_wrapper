use battlemon_near_json_rpc_client_wrapper as rpc_client;
use near_crypto::SecretKey;
use near_jsonrpc_client::NEAR_TESTNET_ARCHIVAL_RPC_URL;
use near_primitives::types::AccountId;
use std::str::FromStr;

#[tokio::test]
async fn call_contract_method_works() {
    let signer = near_crypto::InMemorySigner::from_secret_key(
        AccountId::from_str("battlemon.testnet").unwrap(),
        SecretKey::from_str("ed25519:4czqC7XpYFgtyYMDSxGqPhZaAFFwkwxMgfMT5k2KTjPgfCDfU9MvgxCUoZ3cvr1SKv4F4cpFM183cMtsvNtDY8zd").unwrap(),
    );
    let client = rpc_client::JsonRpcWrapper::connect(NEAR_TESTNET_ARCHIVAL_RPC_URL, signer);
    client
        .call_contract_method(
            AccountId::from_str("nft.dev-20220414034725-94826614851521").unwrap(),
            "update_token_media".to_string(),
            serde_json::json!({
                "token_id": "3",
                "new_media": "Qmb7yyqxh3YmAYkU1rbPwrNtzwbQCia1QUfw5RAeXFyBRu"
            }),
            1_000_000_000,
            1,
        )
        .await
        .unwrap();
}
