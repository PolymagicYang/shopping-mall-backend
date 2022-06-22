use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Good {
    pub goodId: String,
    pub goodImage: String,
    pub goodName: String,
    pub goodValue: String,
    pub goodIntroduction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goods {
    pub goods: Vec<Good> 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodId {
    pub goodId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cart {
    pub goodId: String,
    pub goodImage: String,
    pub goodName: String,
    pub goodValue: String,
    pub goodNumber: String,
    pub isSelected: bool,
}