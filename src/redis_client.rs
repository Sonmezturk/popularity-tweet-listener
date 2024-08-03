extern crate redis;
extern crate serde_json;

use redis::{Commands, Connection, RedisResult};

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref CON: Mutex<Connection> = {
        let client = redis::Client::open("redis://default:<RedisUrl>:12789").unwrap();
        let con = client.get_connection().unwrap();
        Mutex::new(con)
    };
}

pub fn push_to_redis(data: &str) -> RedisResult<()> {
    let mut con = CON.lock().unwrap();

    let _: () = con.sadd("tweet_titles", data.to_string()).unwrap();
    let _: () = con.expire("tweet_titles", 10800).unwrap();
    Ok(())
}

pub async fn get_from_redis() -> RedisResult<Vec<String>> {
    let mut con = CON.lock().unwrap();
    let retrieved_array: Vec<String> = con.smembers("tweet_titles").unwrap();
    Ok(retrieved_array)
}
