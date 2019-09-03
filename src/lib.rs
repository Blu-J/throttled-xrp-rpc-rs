#![allow(non_snake_case)]

#[macro_use]
extern crate throttled_json_rpc;

use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Balance {
    XRP(BigDecimal),
    Other {
        currency: String,
        issuer: String,
        value: BigDecimal,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/**
* Starts with r
* Length is 25-35 chars in length
* 1: https://xrpl.org/basic-data-types.html#addresses
*/
pub struct Account(String);

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BiggDecimal {
    BigDecimal(String),
    I64(i64),
    F64(f64),
}
impl From<BiggDecimal> for BigDecimal {
    fn from(value: BiggDecimal) -> BigDecimal {
        match value {
            BiggDecimal::BigDecimal(x) => panic!(),
            BiggDecimal::I64(x) => x.into(),
            BiggDecimal::F64(x) => x.into(),
        }
    }
}

fn account_validate(s: &str) -> Result<String, String> {
    const MIN_LENGTH: usize = 25;
    const MAX_LENGTH: usize = 35;
    if s.len() < MIN_LENGTH {
        return Err(format!("{:?} is shorter than {} chars ", s, MIN_LENGTH));
    }
    if let Some(first_char) = s.chars().nth(0) {
        if first_char != 'r' {
            return Err(format!("{:?} does not start with r", s));
        }
    }
    if s.len() > MAX_LENGTH {
        return Err(format!("{:?} is longer than {} chars ", s, MAX_LENGTH));
    }
    Ok(s.into())
}

impl FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        account_validate(s).map(|account| Account(account))
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AccountInfoParams<'a> {
    pub account: &'a Account,
    pub strict: bool,

    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
    pub queue: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct AccountTxParams<'a, 'b> {
    pub account: &'a Account,
    pub ledger_index_min: Option<i64>,
    pub ledger_index_max: Option<i64>,
    pub ledger_hash: Option<&'b str>,

    #[serde(flatten)]
    pub ledger_index: Option<LedgerIndex>,

    pub binary: Option<bool>,
    pub forward: Option<bool>,
    pub limit: Option<u64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct LedgerInfoParams {
    pub ledger_hash: Option<String>,
    #[serde(flatten)]
    pub ledger_index: Option<LedgerIndex>,
    pub full: Option<bool>,
    pub accounts: Option<bool>,
    pub transactions: Option<bool>,
    pub expand: Option<bool>,
    pub owner_funds: Option<bool>,
    pub binary: Option<bool>,
    pub queue: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub enum LedgerEntryType {
    AccountRoot, // WHY DOES THIS EVEN EXIST???
}

#[derive(Deserialize, Debug)]
pub struct AccountData {
    pub Account: String,
    pub Balance: BiggDecimal,
    pub Flags: BiggDecimal,
    pub LedgerEntryType: LedgerEntryType,
    pub OwnerCount: BiggDecimal,
    pub PreviousTxnID: String,
    pub PreviousTxnLgrSeq: BiggDecimal,
    pub Sequence: BiggDecimal,
    pub index: String,
}

#[derive(Deserialize, Debug)]
pub struct QueuedTransaction {
    pub LastLedgerSequence: Option<BiggDecimal>,
    pub auth_change: bool,
    pub fee: BiggDecimal,
    pub fee_level: BiggDecimal,
    pub max_spend_drops: BiggDecimal,
    pub seq: BiggDecimal,
}

#[derive(Deserialize, Debug)]
pub struct AccountTransaction {
    pub meta: serde_json::Value,
    pub tx: AccountTransactionTx,
    pub validated: bool,
}

#[derive(Deserialize, Debug)]
pub struct AccountTransactionTx {
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
}

#[derive(Deserialize, Debug)]
pub struct QueueData {
    pub auth_change_queued: bool,
    pub highest_sequence: BiggDecimal,
    pub lowest_sequence: BiggDecimal,
    pub max_spend_drops_total: BiggDecimal,
    pub transactions: Vec<QueuedTransaction>,
    pub txn_count: BiggDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LedgerIndex {
    Current { ledger_current_index: BigDecimal },
    Number { ledger_index: serde_json::Number },
    StrValue { ledger_index: String },
}

#[derive(Deserialize, Debug)]
pub struct AccountInfo {
    pub account_data: Option<AccountData>,
    pub queue_data: Option<LaziedQueueData>,
    pub status: String,
    pub validated: Option<bool>,

    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
}

#[derive(Deserialize, Debug)]
pub struct AccountTx {
    pub account: Account,
    pub ledger_index_min: i64,
    pub ledger_index_max: i64,
    pub limit: i64,
    pub transactions: Vec<AccountTransaction>,
}

/**
 *  Some fields may be omitted because the values are calculated "lazily" by the queuing mechanism. [1]
 * 1: https://xrpl.org/account_info.html
 */
#[derive(Deserialize, Debug)]
pub struct LaziedQueueData {
    pub auth_change_queued: Option<bool>,
    pub highest_sequence: Option<BiggDecimal>,
    pub lowest_sequence: Option<BiggDecimal>,
    pub max_spend_drops_total: Option<BiggDecimal>,
    pub transactions: Option<Vec<QueuedTransaction>>,
    pub txn_count: Option<BiggDecimal>,
}

#[derive(Deserialize, Debug)]
pub struct PathInfo {
    pub currency: String,
    pub issuer: Option<String>,
    #[serde(rename = "type")]
    pub currency_type: BiggDecimal,
    pub type_hex: String,
}

#[derive(Deserialize, Debug)]
pub struct FinalFieldInfo {
    pub Account: Option<String>,
    pub Balance: Option<Balance>,
    pub Flags: isize,
    pub OwnerCount: Option<BiggDecimal>,
    pub Sequence: Option<BiggDecimal>,
}

#[derive(Deserialize, Debug)]
pub struct PreviousFieldInfo {
    pub Balance: Option<Balance>,
    pub Sequence: Option<BiggDecimal>,
}

#[derive(Deserialize, Debug)]
pub struct ModifiedNodeInfo {
    pub FinalFields: FinalFieldInfo,
    pub PreviousFields: Option<PreviousFieldInfo>, // is this really optional ???
    pub LedgerEntryType: String,
    pub LedgerIndex: String,
    pub PreviousTxnID: Option<String>,
    pub PreviousTxnLgrSeq: Option<BiggDecimal>,
}

#[derive(Deserialize, Debug)]
pub struct AffectedNodeInfo {
    pub ModifiedNode: Option<ModifiedNodeInfo>,
}

#[derive(Deserialize, Debug)]
pub struct MetaTxInfo {
    pub AffectedNodes: Vec<AffectedNodeInfo>,
    pub TransactionIndex: BiggDecimal,
    pub TransactionResult: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionInfo {
    pub Account: String,
    pub Amount: Option<Balance>,
    pub Destination: Option<String>,
    pub Fee: BiggDecimal,
    pub Flags: isize,
    pub Paths: Option<Vec<Vec<PathInfo>>>,
    pub SendMax: Option<Balance>,
    pub Sequence: BiggDecimal,
    pub SigningPubKey: String,
    pub TransactionType: String,
    pub TxnSignature: String,
    pub hash: String,
    pub LedgerIndex: Option<String>,
    pub metaData: MetaTxInfo,
    pub validated: Option<bool>, //option of a bool???
}

#[derive(Deserialize, Debug)]
pub struct NestedLedgerInfo {
    pub accepted: bool,
    pub account_hash: String,
    pub close_flags: isize,
    pub close_time: BiggDecimal,
    pub close_time_human: String,
    pub close_time_resolution: BiggDecimal,
    pub closed: bool,
    pub hash: String,
    pub ledger_hash: String,
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
    pub parent_close_time: BiggDecimal,
    pub parent_hash: String,
    pub seqNum: String,
    pub totalCoins: String,
    pub total_coins: BiggDecimal,
    pub transaction_hash: String,
    pub transactions: Option<Vec<TransactionInfo>>,
}

#[derive(Deserialize, Debug)]
pub struct LedgerInfo {
    pub ledger: Option<NestedLedgerInfo>,
    pub ledger_hash: Option<String>,
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
    pub transactions: Option<Vec<TransactionInfo>>,
    pub status: String,
    pub validated: bool,
}

jsonrpc_client!(pub struct XRPClient {
    single:
        pub fn account_info(&self, params: AccountInfoParams) -> Result<AccountInfo>;
        pub fn account_tx(&self, params: AccountTxParams) -> Result<AccountTx>;
        pub fn ledger(&self, params: LedgerInfoParams) -> Result<LedgerInfo>;
    enum:
});

#[test]
fn json_test() {
    let _: LedgerInfo =
        serde_json::from_reader(std::fs::File::open("ledger.json").unwrap()).unwrap();
}

#[test]
fn json_ledger_test() {
    use serde_json::json;
    #[derive(Deserialize)]
    struct RpcResponse<T> {
        pub result: Option<T>,
        pub error: Option<serde_json::Value>,
        pub id: Option<usize>,
    }
    let test_data:RpcResponse<LedgerInfo> = serde_json::from_str(r#"{"result":{"ledger":{"accepted":true,"account_hash":"CFA12FBAFC585D54858874ADACB1003CB4218B010CF5F8AB4C4984B194E95B4B","close_flags":0,"close_time":620860251,"close_time_human":"2019-Sep-03 21:10:51.000000000","close_time_resolution":10,"closed":true,"hash":"30BC3B59A2DCB4BC402637A1DEE3F22C6AC4D09E2CDFCAE8C84F11D7E6E251F5","ledger_hash":"30BC3B59A2DCB4BC402637A1DEE3F22C6AC4D09E2CDFCAE8C84F11D7E6E251F5","ledger_index":"105938","parent_close_time":620860250,"parent_hash":"197B5016B33A79CECA4AA704B534D5999A9674FAD9CBDD82309835D7A784A35F","seqNum":"105938","totalCoins":"99999999522468910","total_coins":"99999999522468910","transaction_hash":"55771C3FB148C470D36B4AE4F91D402F60C39920649C9D2C3E1829104E82654F"},"ledger_hash":"30BC3B59A2DCB4BC402637A1DEE3F22C6AC4D09E2CDFCAE8C84F11D7E6E251F5","ledger_index":105938,"status":"success","validated":true}}"#).unwrap();
    // serde_json::from_value::<RpcResponse<LedgerInfo>>(test_data).expect("ledger");
}
