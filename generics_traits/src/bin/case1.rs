use generics_traits::MyStruct;

fn add_i8(a:i8, b:i8) -> i8 {
    a + b
}
fn add_i32(a:i32, b:i32) -> i32 {
    a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
    a + b
}

fn main() {
    MyStruct::new(10i32);
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20i32, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));
}