use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::panic::panic_any;

use anyhow::{Error, Result};
use serde::Deserialize;
use mysql::*;
use mysql::prelude::*;

const CONFIG_FILE_PATH: &str = "./database.toml";

#[derive(Debug)]
pub struct DB {
    mysql: Option<HashMap<String, Conn>>
    // redis: Option<HashMap<String, u8>>,
}

impl DB {
    pub fn init() -> Self {
        let config = get_config();
        Self {
            mysql: connect_mysql(&config.mysql.unwrap()) 
        }
    }
    pub fn exec_sql<T>(&mut self, db_name: &str, sql: &str) -> Result<Vec<T>>
    where
        T: FromRow
    {
        if let Some(mysql) = self.mysql.as_mut() {
            if let Some(conn) = mysql.get_mut(db_name) {
                let result = conn.query(sql).unwrap();
                return Ok(result);
            }
        }
        Err(Error::msg("aaa"))
    }
}


#[derive(Debug, Deserialize)]
struct Config {
    mysql : Option<Vec<MysqlConnInfo>>
}

#[derive(Debug, Deserialize)]
struct MysqlConnInfo {
    host: String,
    user: String,
    password: String,
    db_list: Vec<String>
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RedisConnInfo {
    host: String,
    port: String,
    db  : String, 
    user: String,
    password: String,
    db_list: Vec<String>
}

fn get_config() -> Config {
    match read_config() {
        Ok(c) => c,
        Err(_)=> panic_any("miss config.toml")
    }
}

fn read_config() -> Result<Config> {
    let mut s = String::new();
    File::open(CONFIG_FILE_PATH)?.read_to_string(&mut s)?;
    Ok(toml::from_str(&s).unwrap())
}

fn connect_mysql(mysql_infos: &Vec<MysqlConnInfo>) -> Option<HashMap<String, Conn>> {
    if mysql_infos.len() == 0 {
        return None;
    }
    let mut conns = HashMap::new();
    for item in mysql_infos {
        //没有数据库需要连接
        if item.db_list.len() == 0 {
            continue;
        }
        for db_name in &item.db_list {
            let opts = OptsBuilder::new()
            .ip_or_hostname(Some(item.host.to_string()))
            .user(Some(item.user.to_string()))
            .pass(Some(item.password.to_string()))
            .db_name(Some(db_name));
            conns.insert(db_name.to_string(), Conn::new(opts).unwrap());
        }
    }
    Some(conns)
}

pub fn init() {
    let mut db = DB {  mysql: None};
    let config = get_config();
    db.mysql = connect_mysql(&config.mysql.unwrap());
    // db.redis = connect_mysql
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init(){
        init(); 
    }
}