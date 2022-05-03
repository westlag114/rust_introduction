// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     //'{}'の長さは、{}です
//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len()メソッドは、Stringの長さを返します

//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &str) -> usize {
//     s.len()
// }

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
