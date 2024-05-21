use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Status {
    #[serde(alias = "STATUS")]
    status: String,
    #[serde(alias = "When")]
    when: u64,
    #[serde(alias = "Code")]
    code: u64,
    #[serde(alias = "Msg")]
    msg: String,
    #[serde(alias = "Description")]
    description: String,
}
#[derive(Debug, Deserialize)]
pub struct Summary {
    #[serde(alias = "Elapsed")]
    pub elapsed: u64,
    #[serde(alias = "MHS av")]
    pub mhs_av: f64,
    #[serde(alias = "MHS 30s")]
    pub mhs30s: f64,
    #[serde(alias = "MHS 1m")]
    pub mhs1m: f64,
    #[serde(alias = "MHS 5m")]
    pub mhs5m: f64,
    #[serde(alias = "MHS 15m")]
    pub mhs15m: f64,
    #[serde(alias = "Found Blocks")]
    found_blocks: u64,
    #[serde(alias = "Getworks")]
    getworks: u64,
    #[serde(alias = "Accepted")]
    accepted: u64,
    #[serde(alias = "Rejected")]
    pub rejected: u64,
    #[serde(alias = "Hardware Errors")]
    hardware_errors: u64,
    #[serde(alias = "Utility")]
    utility: f64,
    #[serde(alias = "Discarded")]
    discarded: f64,
    #[serde(alias = "Stale")]
    stale: f64,
    #[serde(alias = "Get Failures")]
    get_failures: f64,
    #[serde(alias = "Local Work")]
    local_work: f64,
    #[serde(alias = "Remote Failures")]
    remote_failures: f64,
    #[serde(alias = "Network Blocks")]
    network_blocks: f64,
    #[serde(alias = "Total MH")]
    total_mh: f64,
    #[serde(alias = "Work Utility")]
    work_utility: f64,
    #[serde(alias = "Difficulty Accepted")]
    difficulty_accepted: f64,
    #[serde(alias = "Difficulty Rejected")]
    difficulty_rejected: f64,
    #[serde(alias = "Difficulty Stale")]
    difficulty_stale: f64,
    #[serde(alias = "Best Share")]
    best_share: f64,
    #[serde(alias = "Device Hardware%")]
    device_hardware: f64,
    #[serde(alias = "Device Rejected%")]
    device_rejected: f64,
    #[serde(alias = "Pool Rejected%")]
    pool_rejected: f64,
    #[serde(alias = "Pool Stale%")]
    pool_stale: f64,
    #[serde(alias = "Last getwork")]
    lastgetwork: u64,
}

#[derive(Debug, Deserialize)]
pub struct CgStatusResponse {
    pub id: u64,
    #[serde(alias = "STATUS")]
    pub status: Vec<Status>,
    #[serde(alias = "SUMMARY")]
    pub summary: Vec<Summary>,
}

#[derive(Debug, Deserialize)]
pub struct CgVersionResponse {
    pub id: u64,
    #[serde(alias = "STATUS")]
    pub status: Vec<Status>,
    #[serde(alias = "VERSION")]
    pub version: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    #[serde(alias = "CGMiner")] //"4.11.1",
    pub cg_miner: String,
    #[serde(alias = "API")] //"3.7",
    pub api: String,
    #[serde(alias = "STM8")] //"20.08.01",
    pub stm8: String,
    #[serde(alias = "PROD")] //"AvalonMiner 1246-83",
    pub prod: String,
    #[serde(alias = "MODEL")] //"1246-83",
    pub model: String,
    #[serde(alias = "HWTYPE")] //"MM3v2_X3",
    pub hwtype: String,
    #[serde(alias = "SWTYPE")] //"MM314",
    pub swtype: String,
    #[serde(alias = "VERSION")] //"21042001_4ec6bb0_61407fa",
    pub version: String,
    #[serde(alias = "LOADER")] //"d0d779de.00",
    pub loader: String,
    #[serde(alias = "DNA")] //"020100003fc244a1",
    pub dna: String,
    #[serde(alias = "MAC")] //"b4a2eb3526c3",
    pub mac: String,
    #[serde(alias = "UPAPI")] //"2"
    pub upapi: String,
}

#[derive(Debug, Deserialize)]
pub struct CgPoolsResponse {
    pub id: u64,
    #[serde(alias = "STATUS")]
    pub status: Vec<Status>,
    #[serde(alias = "POOLS")]
    pub pools: Vec<Pools>,
}

#[derive(Debug, Deserialize)]
pub struct Pools {
    #[serde(alias = "POOL")] //0,
    pool: u32,
    #[serde(alias = "URL")] //"stratum+tcp://btc-cudc-ru.f2pool.com:3333",
    url: String,
    #[serde(alias = "Status")] //"Alive",
    status: String,
    #[serde(alias = "Priority")] //0,
    priority: u32,
    #[serde(alias = "Quota")] //1,
    quota: u32,
    #[serde(alias = "Long Poll")] //"N",
    long_poll: String,
    #[serde(alias = "Getworks")] //15058,
    getworks: u64,
    #[serde(alias = "Accepted")] //22165,
    accepted: u64,
    #[serde(alias = "Rejected")] //13,
    rejected: u64,
    #[serde(alias = "Works")] //11195678,
    works: u64,
    #[serde(alias = "Discarded")] //0,
    discarded: u64,
    #[serde(alias = "Stale")] //1,
    stale: u32,
    #[serde(alias = "Get Failures")] //1,
    get_failures: u32,
    #[serde(alias = "Remote Failures")] //0,
    remote_failures: u32,
    #[serde(alias = "User")] //"temporal15.k2lx112x5x9",
    user: String,
    #[serde(alias = "Last Share Time")] //600073,
    last_share_time: u64,
    #[serde(alias = "Diff1 Shares")] //11370319874,
    diff1_shares: u64,
    #[serde(alias = "Proxy Type")] //"",
    proxy_type: String,
    #[serde(alias = "Proxy")] //"",
    proxy: String,
    #[serde(alias = "Difficulty Accepted")] //11338121216.00000000,
    difficulty_accepted: f64,
    #[serde(alias = "Difficulty Rejected")] //5767170.00000000,
    difficulty_rejected: f64,
    #[serde(alias = "Difficulty Stale")] //524288.00000000,
    difficulty_stale: f64,
    #[serde(alias = "Last Share Difficulty")] //524288.00000000,
    last_share_difficulty: f64,
    #[serde(alias = "Work Difficulty")] //524288.00000000,
    work_difficulty: f64,
    #[serde(alias = "Has Stratum")] //true,
    has_stratum: bool,
    #[serde(alias = "Stratum Active")] //true,
    stratum_active: bool,
    #[serde(alias = "Stratum URL")] //"btc-cudc-ru.f2pool.com",
    stratum_url: String,
    #[serde(alias = "Stratum Difficulty")] //524288.00000000,
    stratum_difficulty: f64,
    #[serde(alias = "Has Vmask")] //true,
    has_vmask: bool,
    #[serde(alias = "Has GBT")] //false,
    has_gbt: bool,
    #[serde(alias = "Best Share")] //108365198894,
    best_share: u64,
    #[serde(alias = "Pool Rejected%")] //0.0508,
    pool_rejected: f64,
    #[serde(alias = "Pool Stale%")] //0.0046,
    pool_stale: f64,
    #[serde(alias = "Bad Work")] //1004,
    bad_work: f64,
    #[serde(alias = "Current Block Height")] //843864,
    current_block_height: u64,
    #[serde(alias = "Current Block Version")] //536870912
    current_block_version: u64,
}

#[derive(Debug, Deserialize)]
pub struct CgMultiResponse {
    pub summary: Vec<CgStatusResponse>,
    pub pools: Vec<CgPoolsResponse>,
    pub version: Vec<CgVersionResponse>,
}
#[derive(Debug, Default)]
pub struct MetricsPool {
    pub pool: u32,
    pub url: String,
    pub status: String,
    pub user: String,
}

impl MetricsPool {
    fn new(pool: Pools) -> MetricsPool {
        MetricsPool {
            pool: pool.pool,
            url: pool.url,
            status: pool.status,
            user: pool.user,
        }
    }
}

#[derive(Default)]
pub struct MetricsInfo {
    pub mhs_av: f64,
    pub mhs30s: f64,
    pub mhs1m: f64,
    pub mhs5m: f64,
    pub mhs15m: f64,
    pub rejected: f64,
    pub pools: Vec<MetricsPool>,
    pub model: String,
    pub loader: String,
    pub dna: String,
    pub mac: String,
}

impl MetricsInfo {
    pub fn new(cg_ping: CgMultiResponse) -> MetricsInfo {
        MetricsInfo {
            mhs_av: cg_ping.summary[0].summary[0].mhs_av,
            mhs30s: cg_ping.summary[0].summary[0].mhs30s,
            mhs1m: cg_ping.summary[0].summary[0].mhs1m,
            mhs5m: cg_ping.summary[0].summary[0].mhs5m,
            mhs15m: cg_ping.summary[0].summary[0].mhs15m,
            rejected: cg_ping.summary[0].summary[0].rejected as f64,
            pools: cg_ping
                .pools
                .into_iter()
                .map(|pools| pools.pools.into_iter().map(|i| MetricsPool::new(i)))
                .flatten()
                .collect(),
            model: cg_ping.version[0].version[0].model.clone(),
            loader: cg_ping.version[0].version[0].loader.clone(),
            dna: cg_ping.version[0].version[0].dna.clone(),
            mac: cg_ping.version[0].version[0].mac.clone(),
        }
    }
}
