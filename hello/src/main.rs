// fn main() {
//     let a = 10;
//     let b = 15;
//     println!("Hello, world!, {} {}", a, b);
// }

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

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
//     let other_arr: [u8; 5] = [100; 5];
//
//     println!("index: {}, length {}", arr[0], other_arr.len());
//
//     println!("{:?}", other_arr);
// }

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

// fn main() {
//     println!("{}", is_even(2));
// }
//
// pub fn is_even(num: u8) -> bool {
//     let digit: u8 = num % 2;
//     digit == 0
// }

// fn main() {
//     let mut num = 5;
//     num = 3;
//     println!("{}", num);
// }

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

fn main() {
    let n = 3;
    if n > 0 {
        println!("greater than 0");
    } else if n < 0 {
        println!("less than 0");
    } else {
        println!("is 0");
    }
}
