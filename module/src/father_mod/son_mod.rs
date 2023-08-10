

pub fn son_function() {
    println!("use son_function");
}

pub struct SonStruct {
    value: i8
}

impl SonStruct {
    pub fn new(value: i8) -> Self {
        println!("use SonStruct::new,value:{value}");    
        SonStruct{ value }
    }

    pub fn print(&self) {
        println!("FatherStruct::print,value: {}", self.value);
    }
}