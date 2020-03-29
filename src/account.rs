#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::default::{default_string, default_i32};

// 注意当前设置的数据大小 是否可能会出现溢出情况  这需要我们进行考虑
#[allow(dead_code, non_snake_case)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Account {
    user_id: String,
    currency: String,
    pre_balance: f64,
    deposit: f64,
    withdraw: f64,
    WithdrawQuota: f64,
    close_profit: f64,
    commission: f32,
    premium: f32,
    static_balance: f64,
    position_profit: f64,
    float_profit: f64,
    balance: f64,
    margin: f64,
    frozen_margin: f64,
    frozen_commission: f32,
    frozen_premium: f32,
    available: f64,
    risk_ratio: f32,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct BankDetail {
    id: String,
    name: String,
    bank_account: String,
    fetch_amount: f32,
    qry_count: i64,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Order {
    seqno: i32,
    user_id: String,
    order_id: String,
    exchange_id: String,
    instrument_id: String,
    direction: String,
    offset: String,
    volume_orign: i32,
    price_type: String,
    limit_price: f32,
    time_condition: String,
    volume_condition: String,
    insert_date_time: i64,
    exchange_order_id: String,
    status: String,
    volume_left: i32,
    last_msg: String,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Position {
    user_id: String,
    exchange_id: String,
    instrument_id: String,
    volume_long_today: i32,
    volume_long_his: i32,
    volume_long: i32,
    volume_long_frozen_today: i32,
    volume_long_frozen_his: i32,
    volume_long_frozen: i32,
    volume_short_today: i32,
    volume_short_his: i32,
    volume_short: i32,
    volume_short_frozen_today: i32,
    volume_short_frozen_his: i32,
    volume_short_frozen: i32,
    volume_long_yd: i32,
    volume_short_yd: i32,
    pos_long_his: i32,
    pos_long_today: i32,
    pos_short_his: i32,
    pos_short_today: i32,
    open_price_long: f64,
    open_price_short: f64,
    open_cost_long: f64,
    open_cost_short: f64,
    position_price_long: f64,
    position_price_short: f64,
    position_cost_long: f64,
    position_cost_short: f64,
    last_price: f64,
    float_profit_long: f64,
    float_profit_short: f64,
    float_profit: f64,
    position_profit_long: f64,
    position_profit_short: f64,
    position_profit: f64,
    margin_long: f64,
    margin_short: f64,
    margin: f64,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Trade {
    seqno: i32,
    user_id: String,
    trade_id: String,
    exchange_id: String,
    instrument_id: String,
    order_id: String,
    exchange_trade_id: String,
    direction: String,
    offset: String,
    volume: i32,
    price: f64,
    trade_date_time: i64,
    commission: f32,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Transfer {
    datetime: i64,
    currency: String,
    amount: f64,
    error_id: i16,
    error_msg: String,
}

/// QIFI账户数据结构
/// Examples
/// ```
///
/// ```
#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct QIFI {
    #[serde(default = "default_string")]
    pub databaseip: String,
    pub account_cookie: String,
    pub password: String,
    pub portfolio: String,
    pub broker_name: String,
    pub capital_password: String,
    pub bank_password: String,
    pub bankid: String,
    pub investor_name: String,
    pub money: f64,
    pub pub_host: String,
    pub settlement: HashMap<String, String>,
    // 是一个字典是否考虑反序列化
    // #[serde(default="default_i32")]
    pub taskid: String,
    pub trade_host: String,
    pub updatetime: String,
    pub wsuri: String,
    pub bankname: String,
    pub trading_day: String,
    pub status: i16,
    pub accounts: Account,
    // 注意下面都是不确定的
    pub banks: HashMap<String, BankDetail>,
    #[serde(default = "Default::default")]
    pub event: HashMap<String, String>,
    pub orders: HashMap<String, Order>,
    pub positions: HashMap<String, Position>,
    pub trades: HashMap<String, Trade>,
    pub transfers: HashMap<String, Transfer>,
    #[serde(default = "default_i32")]
    pub ping_gap: i32
}
