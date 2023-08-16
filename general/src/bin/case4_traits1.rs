
trait Frame {
    fn get_port_id(&self) -> &String;
    fn print_port_id(&self) {
        println!("Port ID: {}", self.get_port_id());
    }
}


struct CAN {
    port_id: String,
}
impl Frame for CAN {
    fn get_port_id(&self) -> &String {
        &self.port_id
    }
}

struct ETH {
    port_id: String,
}
impl Frame for ETH {
    fn get_port_id(&self) -> &String {
        &self.port_id
    }
}

fn main() {
    let can_frame = CAN {port_id: "CAN123".to_string(),};
    let eth_frame = ETH {port_id: "ETH456".to_string()};

    can_frame.print_port_id(); 
    eth_frame.print_port_id(); 
}
