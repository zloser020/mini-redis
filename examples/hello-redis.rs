use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 对mini-redis的地址打开一个链接
    let mut client = client::connect("127.0.0.1:6379").await?;

    //  设置key:hello value:world
    client.set("hello", "world".into()).await?;

    // 获取value
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
