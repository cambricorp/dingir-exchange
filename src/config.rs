use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MysqlCfg {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub pass: String,
    pub name: String,
    pub charset: String,
}

impl Default for MysqlCfg {
    fn default() -> Self {
        MysqlCfg {
            host: "127.0.0.1".to_string(),
            port: 3306,
            user: "root".to_string(),
            pass: "".to_string(),
            name: "test".to_string(),
            charset: "utf8mb4".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Asset {
    pub name: String,
    pub prec_save: u32,
    pub prec_show: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MarketUnit {
    pub name: String,
    pub prec: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Market {
    pub name: String,
    pub stock: MarketUnit,
    pub money: MarketUnit,
    pub fee_prec: u32,
    pub min_amount: Decimal,
}

impl Default for MarketUnit {
    fn default() -> Self {
        MarketUnit {
            name: "".to_string(),
            prec: 0,
        }
    }
}

impl Default for Market {
    fn default() -> Self {
        Market {
            name: "".to_string(),
            fee_prec: 4,
            min_amount: Decimal::from_str("0.01").unwrap(),
            stock: Default::default(),
            money: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub debug: bool,
    pub db_log: String,
    pub db_history: String,
    pub assets: Vec<Asset>,
    pub markets: Vec<Market>,
    pub brokers: String,
    pub slice_interval: i32,
    pub slice_keeptime: i32,
    pub history_thread: i32,
    pub cache_timeout: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            debug: false,
            db_log: Default::default(),
            db_history: Default::default(),
            assets: Vec::new(),
            markets: Vec::new(),
            brokers: "127.0.0.1:9092".to_string(),
            slice_interval: 86400,
            slice_keeptime: 86400 * 3,
            history_thread: 10,
            cache_timeout: 0.45,
        }
    }
}