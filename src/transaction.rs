use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct Input {
    txid: Txid,
    output_index: u32,
    script: String,
    sequence: u32,
}

impl Input {
    pub fn new(
        txid: Txid,
        output_index: u32,
        script: String,
        sequence: u32
    ) -> Input {
        Input {
            txid,
            output_index,
            script,
            sequence,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Output {
    #[serde(serialize_with = "as_btc")]
    amount: Amount,
    script_pubkey: String,
}

impl Output {
    pub fn new(amount: Amount, script_pubkey: String) -> Output {
        Output {
            amount,
            script_pubkey,
        }
    }
}

fn as_btc<T: BitcoinValue, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    transaction_id: Txid,
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    lock_time: u32,
}

impl Transaction {
    pub fn new(
        transaction_id: Txid,
        version: u32,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
        lock_time: u32
    ) -> Transaction {
        Transaction {
            transaction_id,
            version,
            inputs,
            outputs,
            lock_time,
        }
    }
}

#[derive(Debug)]
pub struct Txid([u8; 32]);

impl Txid {
    pub fn from_bytes(bytes: [u8; 32]) -> Txid {
        Txid(bytes)
    }
}

impl Serialize for Txid {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut bytes = self.0.clone();
        bytes.reverse();
        s.serialize_str(&hex::encode(bytes))
    }
}

#[derive(Debug)]
pub struct Amount(u64);

impl Amount {
    pub fn from_sat(satoshi: u64) -> Amount {
        Amount(satoshi)
    }
}

trait BitcoinValue {
    fn to_btc(&self) -> f64;
}

impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}