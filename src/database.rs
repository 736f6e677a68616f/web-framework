use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::panic::panic_any;

use anyhow::{Error, Result};
use redis::Connection;
use serde::Deserialize;
use mysql::*;
use mysql::prelude::*;

const CONFIG_FILE_PATH: &str = "./database.toml";
#[allow(dead_code)]
pub struct DB {
    mysql: Option<HashMap<String, Conn>>,
    redis: Option<HashMap<String, Connection>>,
}

impl DB {
    pub fn init() -> Self {
        let config = get_config();
        Self {
            mysql: connect_mysql(&config.mysql.unwrap()),
            redis: connect_redis(&config.redis.unwrap()) 
        }
    }
    pub fn exec_sql<T, U, F>(&mut self, db_name: &str, sql: &str, f: F) -> Result<Vec<U>>
    where
        T: FromRow,
        F: FnMut(T) -> U
    {
        if let Some(mysql) = self.mysql.as_mut() {
            if let Some(conn) = mysql.get_mut(db_name) {
                let result = conn.query_map(sql, f).unwrap();
                return Ok(result);
            }
        }
        Err(Error::msg("aaa"))
    }
}


#[derive(Debug, Deserialize)]
struct Config {
    mysql : Option<Vec<MysqlConnInfo>>,
    redis : Option<Vec<RedisConnInfo>>,
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

fn connect_mysql(mysql_info: &Vec<MysqlConnInfo>) -> Option<HashMap<String, Conn>> {
    if mysql_info.len() == 0 {
        return None;
    }
    let mut conns = HashMap::new();
    for item in mysql_info {
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

fn connect_redis(redis_info: &Vec<RedisConnInfo>) -> Option<HashMap<String, Connection>> {
    if redis_info.len() == 0 {
        return None;
    }
    let mut conns = HashMap::new();
    for item in redis_info {
        //没有数据库需要连接
        if item.db_list.len() == 0 {
            continue;
        }
        for db_name in &item.db_list {
            let url = format!("redis://{}:{}@{}:{}/{}", item.user, item.password, item.host, item.port, item.db);
            conns.insert(db_name.to_string(), redis::Client::open(url).unwrap().get_connection().unwrap());
        }
    }
    Some(conns)
}

#[allow(dead_code)]
pub fn init() {
    DB::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init(){
        init(); 
    }
}