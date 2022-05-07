pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!["a", "b", "c", "d"];
    println!("the largest is {}", largest(number_list));
    println!("the largest is {}", largest(char_list));
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
