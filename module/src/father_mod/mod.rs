mod son_mod;
use son_mod::{SonStruct,son_function};

pub fn father_function() {
    println!("use father_function");    
    son_function();
    let my_struct = SonStruct::new(11i8);
    my_struct.print();
}

pub struct FatherStruct {
    value: i8
}

impl FatherStruct {
    pub fn new(value: i8) -> Self {
        println!("use FatherStruct::new,value:{value}");    
        FatherStruct{ value }
    }

    pub fn print(&self) {
        println!("use FatherStruct::print,value: {}", self.value);
    }
}