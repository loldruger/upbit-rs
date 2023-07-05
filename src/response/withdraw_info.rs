use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WithdrawListSource {
    r#type: String,
    uuid: String,
    currency: String,
    net_type: String,
    txid: String,
    state: String,
    created_at: String,
    done_at: String,
    amount: String,
    fee: String,
    transaction_type: String
}

impl WithdrawListSource {
    pub fn r#type(&self) -> String { todo!() }
    pub fn uuid(&self) -> String { todo!() }
    pub fn currency(&self) -> String { todo!() }
    pub fn net_type(&self) -> String { todo!() }
    pub fn txid(&self) -> String { todo!() }
    pub fn state(&self) -> String { todo!() }
    pub fn created_at(&self) -> chrono::NaiveDateTime { todo!() }
    pub fn done_at(&self) -> chrono::NaiveDateTime { todo!() }
    pub fn amount(&self) -> f32 { todo!() }
    pub fn fee(&self) -> f32 { todo!() }
    pub fn transaction_type(&self) -> String { todo!() }
}
