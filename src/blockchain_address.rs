#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddress {
    pub address: String,
    pub txids: Vec<String>,
}
