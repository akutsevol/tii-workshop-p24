mod refs;
use crate::refs::{f1, f2, f3, f4};

fn main() {
    println!("Hello, world!");
    let mut tuple = (10, 20, false);
    let res_1 = f1(&mut tuple);
    println!("{:?}", res_1.unwrap());

    let mut slice = [10, 20, 30, 40];
    let res_2 = f2(&mut slice, 2);
    println!("{:?}", res_2.unwrap());

    let mut slice = [10, 20, 30, 40];
    let res_3 = f3(&mut slice, 1);
    println!("{:?}", res_3.unwrap());

    let slice = [10, 20, 30, 40, 50, 60, 70, 80];
    let res_4 = f4(&slice);
    println!("{:?}", res_4);
}
