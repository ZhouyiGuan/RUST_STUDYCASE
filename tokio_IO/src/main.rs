
use tokio::task;
use tokio::fs::File;
use tokio::net::TcpStream;
use std::sync::{Arc,Mutex};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

struct SharedData<'a> {
    portpath:&'a str,
    logfile_path: &'a str,
    forward_portpath: &'a str,
    addr: &'a str,
    modidata: Mutex<String>
}

async fn write_to_file(receive_data: Arc<Vec<u8>>, shared_data: Arc<SharedData<'_>>) {
    let mut file = File::create(shared_data.portpath).await.expect("Failed to create file");
    file.write_all(&receive_data).await.expect("Failed to write to file");
}

async fn forward_to_another_port(receive_data: Arc<Vec<u8>>, shared_data: Arc<SharedData<'_>>) {
    // port.write_all(&receive_data).await.expect("Failed to forward to another port");
}

async fn send_to_tcp_client(receive_data: Arc<Vec<u8>>, shared_data: Arc<SharedData<'_>>) {
    let mut stream = TcpStream::connect(shared_data.addr).await.expect("Failed to connect to TCP client");
    stream.write_all(&receive_data).await.expect("Failed to send data to TCP client");
}

async fn handle_port(mut port: impl io::AsyncRead + io::AsyncWrite + 'static, mut shared_data: SharedData<'static>){
    //初始化接收缓冲区
    let mut buffer = vec![0u8; 1024];

    // 假设一些共享数据 可以修改
    let shared_data = Arc::new(shared_data);

    // 把串口读写分离 对同一串口的读写可以同时进行 这里的
    let (mut port_reader,mut port_writer) = io::split(port);

    loop{
        // 异步读取数据，也就是.await,有数据唤醒这个任务，没数据就切走让线程执行
        // 其他串口的任务
        let n = port_reader.read(&mut buffer).await.expect("Failed to read from port");

        // 对从串口收到的数据只读
        let receive_data = Arc::new(buffer[..n].to_vec());


        // 创建一个future用来把读到地数据写入文件；
        let receive_data_file = receive_data.clone();
        let shared_data_file = shared_data.clone();
        task::spawn(write_to_file(receive_data_file, shared_data_file));

        // 创建一个future用来转发数据到另一个口；
        let receive_data_forward = receive_data.clone();
        let shared_data_forward = shared_data.clone();
        task::spawn(forward_to_another_port(receive_data_forward, shared_data_forward));

        // 创建一个future来把数据用tcp发送给一个客户端；
        let receive_data_tcp = receive_data.clone();
        let shared_data_tcp = shared_data.clone();
        task::spawn(send_to_tcp_client(receive_data_tcp, shared_data_tcp));

    }
}



#[tokio::main]
async fn main(){
    // 打开串口 以及初始化



    let mut shared_data:SharedData<'_>  = SharedData {
        portpath : "/dev/ttyUSB0",
        logfile_path: "output.txt",
        forward_portpath: "/dev/ttyUSB1",
        addr: "0.0.0.0:8080" ,
        modidata: Mutex::new(String::from("data should be modified"))
    };
    // 为每个串口spawn一个异步任务



    // 创建一个server 把server也spawn进一个任务

}


