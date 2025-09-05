use mini_redis::client;

#[tokio::main]
async fn main() {
    // 与服务器建立连接
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // 产生两个任务，一个get key, 一个set key
    let t1 = tokio::spawn(async {
        let res = client.get("foo").await;
    });

    let t2 = tokio::spawn(async {
        client.set("foo", "bar".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
}