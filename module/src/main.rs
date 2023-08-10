mod father_mod;
use father_mod::{father_function,FatherStruct};

fn main() {
    println!("start program");
    
    father_function();

    FatherStruct::new(11i8).print();
}
