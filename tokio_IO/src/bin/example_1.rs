
use std::sync::{Arc,Mutex};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {   
    // 假设有一个要操作的共享数据
    let v = vec![1, 2, 3];
    let shared_data = Arc::new(Mutex::new(v));

    // 假设有个tcp请求来模拟并发情况
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    loop {
        // 为每个任务获取共享数据；
        let shared_data = shared_data.clone();

        // 异步地等待请求，如果有tcp链接进来了，系统会通知tokio执行这个函数；如果没有就把线程让出
        // 给其他任务
        let (socket, _) = listener.accept().await.unwrap();
        // 不是简单地await每个请求,而是用spawn来生产一个异步任务;这些任务会在加入到tokio地调度
        // 器中,由调度器来决定把任务放到哪个线程上执行.以及什么时候调用任务,什么时候任务阻塞暂停
        // 任务
        tokio::spawn(async move {
            process(socket,shared_data).await;
        });
    }
}

async fn process(socket: TcpStream,shared_data: Arc<Mutex<Vec<i32>>>) {
    //对socket操作..

    //对共享数据操作..
    println!("{:?}",shared_data);
}
