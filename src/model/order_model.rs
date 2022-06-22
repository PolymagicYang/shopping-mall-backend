use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub orderId: Option<String>,
    pub receiver: String,
    pub address: String,
    pub phone: String,
    pub orderMoney: String,
    pub orderState: String,
}
