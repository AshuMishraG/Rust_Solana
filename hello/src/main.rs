// Hello world

// fn main() {
//     let a = 10;
//     let b = 15;
//     println!("Hello, world!, {} {}", a, b);
// }

// First Build

// fn main() {
//     let unsigned: u8 = 10;
//     let signed: i8 = -10;
//     let float: f32 = 1.2;
//
//     println!("unsigned: {} sign: {} float: {}", unsigned, signed, float);
//
//     let letter = "c";
//     let emoji = "\u{1F600}";
//
//     println!("letter: {}, emoji: {}", letter, emoji);
//
//     let is_true = true;
//
//     println!("isTrue: {}", is_true);
// }

// Array

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
//     let other_arr: [u8; 5] = [100; 5];
//
//     println!("index: {}, length {}", arr[0], other_arr.len());
//
//     println!("{:?}", other_arr);
// }

// Tuple

// fn main() {
//     let tuple: (u8, bool, f32) = (5, true, 2.1);
//     let tuple2 = (3, 5);
//
//     println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
//     println!("{:?}", tuple2);
//
//     let (a, b, c) = tuple;
//
//     println!("first {}, second {}, third {}", a, b, c);
// }

// Function

// fn main() {
//     println!("{}", is_even(2));
// }
//
// pub fn is_even(num: u8) -> bool {
//     let digit: u8 = num % 2;
//     digit == 0
// }

// Mutable Function

// fn main() {
//     let mut num = 5;
//     num = 3;
//     println!("{}", num);
// }

// Slice

// fn main() {
//     let arr = [0, 1, 2, 3];
//     let slice = &arr[1..3];
//     borrowing_slice(arr, slice);
// }
//
// fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
//     println!("{:?}", arr);
//     println!("{:?}", slice);
//     println!("length: {}", slice.len());
//     println!("{} {}", slice[0], slice[1]);
// }

// String

// fn main() {
//     let str: &str = "hello world";
//     let mut string: String = String::from("Hello world");
//
//     let slice = &string[..6];
//     slice.len();
//
//     string.push('1');
//     string.push_str("! Bob");
//     string = string.replace("Hello", "Bye");
//     println!("{}", string);
// }

// For loop

// fn main() {
//     let n = 3;
//     if n > 0 {
//         println!("greater than 0");
//     } else if n < 0 {
//         println!("less than 0");
//     } else {
//         println!("is 0");
//     }
// }

// fn main() {
//     for i in 0..6 {
//         println!("{}", i);
//     }
// }

// While loop

// fn main() {
//     let mut i = 0;
//     while i < 4 {
//         println!("{}", i);
//         i += 1;
//         if i == 3 {
//             println!("exit");
//             break;
//         }
//     }
// }

// Match

// fn main() {
//     let i = 4;
//     match i {
//         0 => println!("0"),
//         1 | 2 => println!("1,2"),
//         3..=4 => println!("3,4"),
//         _ => println!("default"),
//     }
// }

// Struct (Classes)

// fn main() {
//     let name = String::from("Bird");
//     let bird = Bird { name, attack: 5 };
//     bird.print_name();
// }
//
// struct Bird {
//     name: String,
//     attack: u64,
// }
//
// impl Bird {
//     fn print_name(&self) {
//         println!("{}", self.name);
//     }
// }

// Traits (interfaces)

// fn main() {
//     let name = String::from("Bird");
//     let bird = Bird { name, attack: 5 };
//     bird.print_name();
//     println!("{} {}", bird.can_fly(), bird.is_animal());
// }
//
// struct Bird {
//     name: String,
//     attack: u64,
// }
//
// impl Bird {
//     fn print_name(&self) {
//         println!("{}", self.name);
//     }
// }
//
// impl Animal for Bird {
//     fn can_fly(&self) -> bool {
//         true
//     }
//     fn is_animal(&self) -> bool {
//         false
//     }
// }
//
// trait Animal {
//     fn can_fly(&self) -> bool;
//     fn is_animal(&self) -> bool {
//         true
//     }
// }

// Enum

// fn main() {
//     let a = MyEnum::A;
//     let b = MyEnum::B(5);
//     let c = MyEnum::C { x: 10, y: 20 };
//
//     println!("{:?}", a);
//     println!("{:?}", b);
//     println!("{:?}", c);
//
//     if let MyEnum::B(val) = b {
//         println!("{}", val);
//     }
//
//     if let MyEnum::C { x, y } = c {
//         println!("{} {}", x, y);
//     }
// }
//
// #[derive(Debug)]
// enum MyEnum {
//     A,
//     B(i32),
//     C { x: i32, y: i32 },
// }

// Vector

// fn main() {
//     let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
//     vec.len();
//     vec[0];
//     vec.push(6);
//     vec.remove(0);
//     println!("{:?}", vec);
// }

// Hash Maps

// use std::collections::HashMap;
// fn main() {
//     let mut map = HashMap::new();
//     map.insert(0, "H1");
//     map.insert(1, "H2");
//     println!("{:?}", map);
//
//     match map.get(&0) {
//         Some(str) => println!("{}", str),
//         None => println!("Dosn't exist in map"),
//     }
//
//     match map.get(&2) {
//         Some(str) => println!("{}", str),
//         _ => println!("Dosn't exist in map"),
//     }
//
//     map.remove(&0);
//     println!("{:?}", map);
// }

// Options

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());
}
