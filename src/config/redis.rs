use crate::config::config::SYSTEM_CONFIG;
use crate::error::Error;
use lazy_static::lazy_static;
use redis::{Commands, FromRedisValue, RedisResult, ToRedisArgs};
use std::collections::HashMap;

fn redis_connect() -> RedisResult<redis::Connection> {
    REDIS_CLIENT.get_connection()
}
pub fn set_redis_obj<T: ToRedisArgs>(key: &str, value: T) -> RedisResult<()> {
    let mut con = redis_connect()?;
    con.set(key, value)
}
pub fn get_redis_obj<T: FromRedisValue>(key: &str) -> RedisResult<T> {
    let mut con = redis_connect()?;
    con.get(key)
}
pub fn expire_redis(key: &str, time: i64) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    con.expire(key, time)
}

pub fn expire_at_redis(key: &str, time: i64) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    con.expire_at(key, time)
}

pub fn get_expire_redis(key: &str) -> RedisResult<i64> {
    let mut con = redis_connect()?;
    con.ttl(key)
}
pub fn del_redis(key: &str) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    con.del(key)
}

pub fn batch_del_redis(key: Vec<&str>) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    let res: i64 = con.del(key)?;
    Ok(res > 0)
}

pub fn exists_redis(key: &str) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    con.exists(key)
}
pub fn set_redis_list<T: ToRedisArgs>(key: &str, value: Vec<T>) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    let mut count = 0;
    for v in value {
        let _: () = con.rpush(key, v)?;
        count += 1;
    }
    Ok(count > 0)
}
pub fn get_redis_obj_list<T: FromRedisValue>(key: Vec<&str>) -> RedisResult<Vec<T>> {
    let mut con = redis_connect()?;
    let mut result = Vec::new();
    for k in key {
        let r: T = con.get(k)?;
        result.push(r);
    }
    Ok(result)
}
pub fn set_redis_set<T: ToRedisArgs>(key: &str, value: Vec<T>) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    let mut count = 0;
    for v in value {
        let _: () = con.sadd(key, v)?;
        count += 1;
    }
    Ok(count > 0)
}
pub fn get_redis_set<T: FromRedisValue>(key: &str) -> RedisResult<Vec<T>> {
    let mut con = redis_connect()?;
    con.smembers(key)
}

pub fn set_redis_zset<V: ToRedisArgs, S: ToRedisArgs>(
    key: &str,
    value: V,
    score: S,
) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    con.zadd(key, score, value)
}

pub fn del_redis_zset(key: &str, value: &str) -> RedisResult<bool> {
    let mut con = redis_connect()?;
    con.zrem(key, value)
}

pub fn get_redis_zset_size(key: &str) -> RedisResult<i64> {
    let mut con = redis_connect()?;
    con.zcard(key)
}

pub fn get_redis_zset_range<V: FromRedisValue>(
    key: &str,
    start: isize,
    stop: isize,
) -> RedisResult<Vec<V>> {
    let mut con = redis_connect()?;
    con.zrange(key, start, stop)
}

pub fn get_redis_zset_range_by_score<V: FromRedisValue>(
    key: &str,
    min: f64,
    max: f64,
    offset: usize,
    count: usize,
) -> RedisResult<Vec<V>> {
    let mut con = redis_connect()?;
    con.zrangebyscore(key, min, max)
        .map(|v: Vec<V>| v.into_iter().skip(offset).take(count).collect())
}

pub fn set_redis_map<V: ToRedisArgs>(key: &str, value: HashMap<&str, V>) -> Result<bool, Error> {
    let mut con = redis_connect()?;
    let mut count = 0;
    for (k, v) in value {
        let _: () = con.hset(key, k, v)?;
        count += 1;
    }
    Ok(count > 0)
}
pub fn get_redis_map<V: FromRedisValue>(
    key: &str,
    field: Vec<&str>,
) -> Result<HashMap<String, V>, Error> {
    let mut con = redis_connect()?;
    let mut result = HashMap::new();
    for f in field {
        let r: V = con.hget(key, f)?;
        result.insert(f.to_string(), r);
    }
    Ok(result)
}

lazy_static! {
    pub static ref REDIS_CLIENT: redis::Client =
        redis::Client::open(&*SYSTEM_CONFIG.app.redis.url).expect("redis connect fail");
}

#[test]
fn redis_test() {
    let key = "test";
    let value = "test";
    set_redis_obj(key, value).unwrap();
    let res: String = get_redis_obj(key).unwrap();
    assert_eq!(res, value);
    del_redis(key).unwrap();
    let res = exists_redis(key);
    assert_eq!(res.unwrap(), false);
}
