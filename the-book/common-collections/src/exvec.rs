fn mean(list: &Vec<i32>) -> f32 {
	let mut sum = 0;
	for v in list {
		sum += v;
	}
	sum as f32 / list.len() as f32
}

fn median(list: &Vec<i32>) -> f32 {
	let mut temp = list.clone();
	temp.sort();
	if list.len() % 2 == 0 {
		let i = list.len() / 2;
		let n = temp[i] as f32;
		let d = temp[i - 1] as f32;
		(n + d) / 2.0
	} else {
		temp[list.len() / 2] as f32
	}
}

use std::collections::HashMap;

fn mode(list: &Vec<i32>) -> i32 {
	let mut map = HashMap::new();
	for i in list {
		let occ = map.entry(i).or_insert(0);
		*occ += 1;
	}

	if let Some(value) = map.iter().max_by(|a, b| a.1.cmp(b.1)) {
		**value.0
	} else {
		0
	}
}

pub fn run() {
	let numbers = vec![1, 3, 6, 4, 2, 2, 7];
	println!("Agerage {}", mean(&numbers));
	println!("Median {}", median(&numbers));
	println!("Mode {}", mode(&numbers));

	let numbers = vec![1, 3, 6, 4, 9, 2, 7, 9];
	println!("Agerage {}", mean(&numbers));
	println!("Median {}", median(&numbers));
	println!("Mode {}", mode(&numbers));
}
