use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //建立mini-redis服务器连接
    let mut new_client = client::connect("127.0.0.1:6379").await?;

    //设置key："hello" 和 值："world"
    new_client.set("hello", "world".into()).await?;

    //获取key = hello 的值
    let result = new_client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result);

    Ok(())
}
