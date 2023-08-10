use super::MyStruct;

impl MyStruct {
    pub fn new(value: i32) -> Self {
        println!("succcess new MyStruct with value {value}");
        MyStruct { value }
    }

    pub fn print(&self) {
        println!("Value: {}", self.value);
    }
}
