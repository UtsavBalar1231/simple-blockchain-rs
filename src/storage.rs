use crate::block::*;
use crate::client::*;

use rocksdb::DB;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;

pub fn add_block(db: &DB, block: &Block) -> Result<(), &'static str> {
    let json = serde_json::to_string(block).map_err(|e| e).unwrap();
    db.put(block.block_hash.clone(), json)
        .map_err(|e| e)
        .unwrap();
    return Ok(());
}

pub fn get_block(db: &DB, block_hash: &String) -> Result<Option<Block>, &'static str> {
    match db.get(block_hash.clone()).expect("Error getting block") {
        Some(block) => {
            let block_s = String::from_utf8(block).map_err(|e| e).unwrap();
            return Ok(Some(serde_json::from_str(&block_s).map_err(|e| e).unwrap()));
        }
        None => return Ok(None),
    }
}

pub fn get_latest_block_number(db: &DB) -> Result<usize, &'static str> {
    match get_latest_block_hash(&db).unwrap() {
        Some(block_hash) => {
            let block_number = get_block_height(&db, &block_hash)?.unwrap();
            return Ok(block_number);
        }
        None => return Ok(0),
    }
}

pub fn set_latest_block(db: &DB, block_hash: &String, height: usize) -> Result<(), &'static str> {
    db.put(b"latest_block_hash", block_hash.clone())
        .map_err(|e| e)
        .unwrap();
    db.put(block_hash.clone(), height.to_string())
        .map_err(|e| e)
        .unwrap();
    return Ok(());
}

pub fn get_latest_block_hash(db: &DB) -> Result<Option<String>, &'static str> {
    match db
        .get(b"latest_block_hash")
        .expect("Error getting latest block hash")
    {
        Some(hash) => return Ok(Some(String::from_utf8(hash).map_err(|e| e).unwrap())),
        None => return Ok(None),
    };
}

pub fn get_block_height(db: &DB, block_hash: &String) -> Result<Option<usize>, &'static str> {
    match db
        .get(block_hash.clone())
        .expect("Error getting block height")
    {
        Some(height) => {
            let height_s = String::from_utf8(height).map_err(|e| e).unwrap();
            let height: usize = height_s.parse().unwrap();
            return Ok(Some(height));
        }
        None => return Ok(None),
    }
}

pub fn set_balance(db: &DB, public_key: PublicKey, balance: f64) -> Result<(), &'static str> {
    db.put(public_key.to_string(), balance.to_string())
        .map_err(|e| e)
        .unwrap();
    return Ok(());
}

pub fn get_balance(db: &DB, public_key: PublicKey) -> Result<Option<f64>, &'static str> {
    match db
        .get(public_key.to_string())
        .expect("Error getting balance")
    {
        Some(balance) => {
            let balance_s = String::from_utf8(balance).map_err(|e| e).unwrap();
            let balance: f64 = balance_s.parse().unwrap();
            return Ok(Some(balance));
        }
        None => return Ok(None),
    };
}

pub fn get_balances(db: &DB) -> Result<HashMap<PublicKey, f64>, &'static str> {
    let mut balances = HashMap::new();
    let mut iter = db.raw_iterator();
    iter.seek_to_first();
    while iter.valid() {
        let public_key = String::from_utf8(iter.key().unwrap().to_vec())
            .map_err(|e| e)
            .unwrap();
        let balance_s = String::from_utf8(iter.value().unwrap().to_vec())
            .map_err(|e| e)
            .unwrap();
        let balance: f64 = balance_s.parse().unwrap();
        balances.insert(PublicKey::from_str(&public_key).unwrap(), balance);
        iter.next();
    }
    return Ok(balances);
}
