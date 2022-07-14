use crossbeam::channel;
use std::collections::HashMap;
use std::io;
use std::thread;

fn main() {
    println!("Hello, factorization!");
    let mut input_vec = Vec::<u64>::new();
    loop {
        let mut input = String::new();
        let bytes = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }
        let input = input.trim().parse();
        match input {
            Ok(i) => input_vec.push(i),
            Err(_e) => {
                println!("There was an error. Please type a number!")
            }
        }
    }
    let mut results = HashMap::<u64, Vec<u64>>::new();
    let mut handles = vec![];

    let (sender, receiver) = channel::unbounded();

    let num = num_cpus::get();

    for _ in 0..num {
        let cloned_receiver = receiver.clone();
        let handle = thread::spawn(move || {
            let mut result = HashMap::<u64, Vec<u64>>::new();
            loop {
                match cloned_receiver.recv() {
                    Err(_e) => {
                        break;
                    }
                    Ok(value) => {
                        let factorized = factorize(value);
                        result.insert(value, factorized);
                    }
                }
            }
            result
        });
        handles.push(handle);
    }

    for i in &input_vec {
        sender.send(*i).unwrap();
    }

    drop(receiver);
    drop(sender);

    for handle in handles {
        let result = handle.join().unwrap();
        results.extend(result);
    }

    for number in input_vec {
        println!(
            "{}",
            results
                .get(&number)
                .unwrap()
                .iter()
                .map(|n| (*n).to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}

fn factorize(number: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut current_number = number;
    let mut time_to_break_loop = false;
    loop {
        let end_of_range = (current_number as f64).sqrt() as u64 + 1;
        for i in 2..end_of_range + 1 {
            if current_number == i {
                time_to_break_loop = true;
                break;
            }
            if current_number % i == 0 {
                let divided = current_number / i;
                current_number = divided;
                result.push(i);
                break;
            }
            if i == end_of_range {
                time_to_break_loop = true;
            }
        }
        if time_to_break_loop {
            result.push(current_number);
            break;
        }
    }
    result.sort_unstable();
    result
}

#[test]
fn test_prime_number() {
    assert_eq!(factorize(5), [5]);
}

#[test]
fn test_complex_number() {
    assert_eq!(factorize(6), [2, 3]);
}

#[test]
fn test_complex_number_with_repeating_primes() {
    assert_eq!(factorize(18), [2, 3, 3]);
}
