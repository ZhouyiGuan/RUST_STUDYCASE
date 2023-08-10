/*
基本文件夹格式：main只会调用father_mod，然后再father_mod文件夹下再存自己
模块的实现，然后这个文件夹下的根文件叫mod，其他自己的子模块也可以再建文件夹
递归下去，也可以就其他名字，自己直接mod，use就行
所有都设为pub，然后再use father_mod::{father_function,FatherStruct}这里
向外暴露接口。因为父类无法访问子类的私有项，所以都设成pub，但是在use处限定别
人能用什么。
*/


mod father_mod;
use father_mod::{father_function,FatherStruct};

fn main() {
    println!("start program");

    father_function();

    FatherStruct::new(11i8).print();
}
