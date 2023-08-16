#![allow(unused_variables)]

//1 泛型函数
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

//2 泛型结构
#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}

//3 为泛型结构编写泛型函数
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//4 为特定类型定义单独的处理函数
impl Point<i128,i128> {
    fn unique_func_i8 (&self) {
        println!("specified func for i8,i8");
    }
}

//5 const泛型 也就是这个泛型的N针对的是值的泛型
fn display_arr<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", arr);
}


fn main() {
    //1
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));

    //2
    let point1 = Point { x: 5i8, y: 10i8 };
    let point2 = Point { x: 5i8, y: true };

    //3
    let point3 = point1.mixup(point2);
    println!("point3 = {:?}",point3);

    //4
    let point4 = Point { x: 10i128, y: 10i128 };
    point4.unique_func_i8();

    //5
    let arr: [i32; 3] = [1, 2, 3];
    display_arr(&arr);
    let arr:[char; 2] = ['a','2'];
    display_arr(&arr);
}