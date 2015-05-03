#![feature(old_io)]
#![feature(collections)]

use std::old_io as io;
use std::thread;

fn merge<E>(left: &mut Vec<E>, right: &mut Vec<E>) -> Vec<E>
        where E: PartialOrd + Clone {
    let mut result = Vec::new();

    while !left.is_empty() && !right.is_empty() {
        if left[0] < right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }

    result.push_all(&left);
    result.push_all(&right);

    return result;
}

fn mergesort<E>(elements: Vec<E>) -> Vec<E>
        where E: PartialOrd + Send + Sync + Clone {
    if elements.len() <= 1 {
        return elements;
    }

    let (left_slice, right_slice) = elements.split_at(elements.len() / 2);

    let mut left = left_slice.to_vec();
    let mut right = right_slice.to_vec();

    if elements.len() < 10 {
        left = mergesort(left);
        right = mergesort(right);
    } else {
        let merge_left = thread::scoped(move || { mergesort(left) });
        right = mergesort(right);

        left = merge_left.join();
    }

    return merge(&mut left, &mut right);
}

fn main() {
    let mut stdin = io::stdin();

    let elements = stdin.lock().lines().map(|result| {
        match result {
            Ok(string) => match string.trim().parse::<i32>() {
                Ok(number) => number,
                Err(e) => panic!(e)
            },
            Err(e) => panic!(e)
        }
    }).collect();

    for element in mergesort(elements) {
        println!("{}", element)
    }
}
