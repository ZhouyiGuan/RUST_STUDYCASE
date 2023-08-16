use tokio::time::{sleep, Duration};

async fn sing() {
    println!("start sing  ...");
    sleep(Duration::from_millis(100)).await;
    println!("end sing  ...");
}
async fn dance() {
    println!("start dance ...");
    println!("end dance ...");
}


// 虽然定义的函数是async，但是代码其实都是同步执行的，只有在里面的多个future都定义好，并且再调用await
// 去等待所有future，才会把所有定义好的future异步执行；这里只是这些future之间是异步，执行顺序会变，其他
// 部分内容都不变
// 
// 这个函数换成同步函数main也行，只是不能使用tokio::main的宏了，但是有另一套写法：
// 
// fn main() {
//     let runtime = tokio::runtime::Runtime::new().unwrap();
//     runtime.block_on(async_main());
// }
// 
// 这一套写法就是同步函数中调用一个异步函数，然后block去等这个异步函数。

#[tokio::main]
async fn main() {
    println!("start async_main()");

    let future1 = sing();
    let future2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。
    // 若`dance`也变成阻塞状态，那`sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(future1, future2);

    sleep(Duration::from_millis(100)).await;

    println!("start async_main()");
}