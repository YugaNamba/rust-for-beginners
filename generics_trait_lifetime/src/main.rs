use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&number_list));

    let number_list = vec![100.24, 30.5, 20.5, 10.5, 5.5];

    println!("The largest number is {}", largest(&number_list));
}
