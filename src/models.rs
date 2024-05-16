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
