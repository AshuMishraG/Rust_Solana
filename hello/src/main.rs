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

//

fn main() {
    let i = 4;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        _ => println!("default"),
    }
}
