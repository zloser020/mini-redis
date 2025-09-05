use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("listening");
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // 这个 `Connection` 让我们读/写Redis **帧(frames)** 而不是
    // 比特流. 这个 `Connection` 类型是由 mini-redis 定义的。
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}

// fn new_sharded_db(num_shards: usize) -> ShardedDb {
//     let mut db = Vec::with_capacity(num_shards);
//     for _ in 0..num_shards {
//         db.push(Mutex::new(HashMap::new()));
//     }
//     Arc::new(db)
// }

// struct CanIncrement {
//     mutex: Mutex<i32>,
// }

// impl CanIncrement {
//     fn increment(&self) {
//         let mut lock = self.mutex.lock().unwrap();
//         *lock += 1;
//     }
// }

// async fn increment_and_do_stuff(can_incr: &CanIncrement) {
//     can_incr.increment();
//     do_something_async().await;
// }

// async fn do_something_async() {

// }