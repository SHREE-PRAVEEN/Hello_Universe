use redis::Client as RedisClient;

pub fn build_redis_client(redis_url: &str) -> anyhow::Result<RedisClient> {
    Ok(RedisClient::open(redis_url)?)
}
