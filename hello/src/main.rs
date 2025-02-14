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

fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length {}", arr[0], other_arr.len());

    println!("{:?}", other_arr);
}
