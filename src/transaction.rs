use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct Input {
    txid: String,
    output_index: u32,
    script: String,
    sequence: u32,
}

impl Input {
    pub fn new(txid: String, output_index: u32, script: String, sequence: u32) -> Input {
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
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

impl Transaction {
    pub fn new(version: u32, inputs: Vec<Input>, outputs: Vec<Output>) -> Transaction {
        Transaction {
            version,
            inputs,
            outputs,
        }
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