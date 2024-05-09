use serde::Deserialize;

/*{
  "STATUS": [
    {
      "STATUS": "S",
      "When": 161232,
      "Code": 11,
      "Msg": "Summary",
      "Description": "cgminer 4.11.1"
    }
  ],
  "SUMMARY": [
    {
      "Elapsed": 161218,
      "MHS av": 88027946.77,
      "MHS 30s": 87240933.02,
      "MHS 1m": 87801784.43,
      "MHS 5m": 87831734.35,
      "MHS 15m": 87908560.29,
      "Found Blocks": 0,
      "Getworks": 5745,
      "Accepted": 11330,
      "Rejected": 5,
      "Hardware Errors": 0,
      "Utility": 4.22,
      "Discarded": 851964100,
      "Stale": 4,
      "Get Failures": 0,
      "Local Work": 3265864,
      "Remote Failures": 0,
      "Network Blocks": 268,
      "Total MH": 14185230482682,
      "Work Utility": 1229760.34,
      "Difficulty Accepted": 3317465088,
      "Difficulty Rejected": 1572864,
      "Difficulty Stale": 0,
      "Best Share": 12764451807,
      "Device Hardware%": 0,
      "Device Rejected%": 0.0476,
      "Pool Rejected%": 0.0474,
      "Pool Stale%": 0,
      "Last getwork": 0
    }
  ],
  "id": 1
}
*/
#[derive(Debug, Deserialize)]
struct Status {
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
struct Summary {
    #[serde(alias = "Elapsed")]
    elapsed: u64,
    #[serde(alias = "MHS av")]
    mh_sav: f64,
    #[serde(alias = "MHS 30s")]
    mhs30s: f64,
    #[serde(alias = "MHS 1m")]
    mhs1m: f64,
    #[serde(alias = "MHS 5m")]
    mhs5m: f64,
    #[serde(alias = "MHS 15m")]
    mhs15m: f64,
    #[serde(alias = "Found Blocks")]
    found_blocks: u64,
    #[serde(alias = "Getworks")]
    getworks: u64,
    #[serde(alias = "Accepted")]
    accepted: u64,
    #[serde(alias = "Rejected")]
    rejected: u64,
    #[serde(alias = "Hardware Errors")]
    hardware_errors: u64,
    #[serde(alias = "Utility")]
    utility: f64,
    #[serde(alias = "Discarded")]
    discarded: u64,
    #[serde(alias = "Stale")]
    stale: u64,
    #[serde(alias = "Get Failures")]
    get_failures: u64,
    #[serde(alias = "Local Work")]
    local_work: u64,
    #[serde(alias = "Remote Failures")]
    remote_failures: u64,
    #[serde(alias = "Network Blocks")]
    network_blocks: u64,
    #[serde(alias = "Total MH")]
    total_mh: u64,
    #[serde(alias = "Work Utility")]
    work_utility: f64,
    #[serde(alias = "Difficulty Accepted")]
    difficulty_accepted: u64,
    #[serde(alias = "Difficulty Rejected")]
    difficulty_rejected: u64,
    #[serde(alias = "Difficulty Stale")]
    difficulty_stale: u64,
    #[serde(alias = "Best Share")]
    best_share: u64,
    #[serde(alias = "Device Hardware%")]
    device_hardware: f64,
    #[serde(alias = "Device Rejected%")]
    device_rejected: f64,
    #[serde(alias = "Pool Rejected%")]
    pool_rejected: f64,
    #[serde(alias = "Pool Stale%")]
    pool_stale: u64,
    #[serde(alias = "Last getwork")]
    lastgetwork: u64,
}

#[derive(Debug, Deserialize)]
pub struct CgStatusResponse {
    id: u64,
    #[serde(alias = "STATUS")]
    status: Vec<Status>,
    #[serde(alias = "SUMMARY")]
    summary: Vec<Summary>,
}
