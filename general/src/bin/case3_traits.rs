
pub struct CAN {
    pub port_id: String, 
    pub size: String, 
    pub data: String, 
}
pub struct LIN {
    pub port_id: String,
    pub content: String
}
pub struct ETH {
    pub port_id: String,
    pub data: String
}

//1 可以在特征中定义一个默认实现方法（也就是基类虚函数）
//当我有这个特征就可以使用这个方法
pub trait Frame {
    fn frame_type(&self) -> String {
        String::from("default frame type")
    }
}
//CAN有这个特征，且用的是默认的实现方法
impl Frame for CAN {}
//LIN也有这个特征 但是选择自己定义实现方法（重载虚函数）
impl Frame for LIN {
    fn frame_type(&self) -> String {
        String::from("LIN frame type")
    }
}
//假设ETH没有实现这个特征


//2 特征作为函数值：只有具有这个特征的结构可以使用这个函数
pub fn func_only_for_frame(item: &impl Frame) { 
    println!("func_only_for_frame: {}", item.frame_type());//这个函数也只能使用相对应特征的内容
}


fn main() {
    let can_frame = CAN{port_id: "can_port_id".to_string(),size: "can_size".to_string(), data: "can_data".to_string()};
    let lin_frame = LIN{port_id: "lin_port_id".to_string(),content: "lin_content".to_string()};
    let eth_frame = ETH{port_id: "eth_port_id".to_string(),data: "eth_data".to_string()};

    //1
    println!("{}",can_frame.frame_type());
    println!("{}",lin_frame.frame_type());

    //2
    func_only_for_frame(&can_frame);
    // func_only_for_frame(&eth_frame); //这里eth_frame没法使用这个函数 因为我们在上面没有给他这个frame的特征
}