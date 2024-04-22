#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockbookInfo {
    pub coin: String,
    // host: String,
    // version: String,
    // git_commit: String,
    // build_time: String,
    // sync_mode: bool,
    // initial_sync: bool,
    // in_sync: bool,
    // best_height: i64,
    // last_block_time: String,
    // in_sync_mempool: bool,
    // last_mempool_time: String,
    // mempool_size: i64,
    // decimals: i64,
    // db_size: i64,
    // about: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub chain: String,
    // blocks: i64,
    // headers: i64,
    // best_block_hash: String,
    // difficulty: String,
    // size_on_dict: i64,
    // version: String,
    // subversion: String,
    // protocol_version: String,
}

pub struct BlockchainStatus {
    pub blockchain: Blockbook,
    pub backend: Backend,
}
