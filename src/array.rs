extern crate rand;

use self::rand::Rng;
use super::sort;

pub fn check_sorted(a: &[i32]) -> bool {
	//let mut ret = true;
	for i in 1..a.len() {
		if a[i] < a[i-1] {
			/*println!("Sort check failed, a[{}]={} > a[{}] = {}", 
				i, a[i], i-1, a[i-1]);*/
			return false;
		}
	}

	return true		// Return this value
}

pub fn fill_random(a: &mut [i32]) {
	let mut rng = rand::thread_rng();

	for i in 0..a.len() {
		a[i] = rng.gen::<i32>();
	}
}

pub fn fill_sorted(a: &mut [i32]) {
	let mut i = 0;
	let mut j = 0;
	let mut rng = rand::thread_rng();

	while i < a.len() {
		if rng.gen::<i8>() > 0 {
			a[i] = j;
			i += 1;
		}

		j += 1;
	}
}

pub fn fill_equal(a: &mut [i32]) {
	let total = ((a.len() as f64).sqrt() as usize);
	let mut rng = rand::thread_rng();

	for i in 0..total {
		let val = rng.gen::<i32>();
		for j in i*total..(i+1)*total {
			a[j] = val;
		}
	}

	shuffle(a);
}

pub fn fill_almost_sorted(a: &mut [i32]) {
	fill_sorted(a);

	let mut count = 0;
	while count < ((a.len() as f64).log2() as usize) {
		let mut rng = rand::thread_rng();

		let first: u32 = rng.gen_range(0, (a.len() as u32));
		let second: u32 = rng.gen_range(0, (a.len() as u32));
		swap(a, first as usize, second as usize);

		count += 1;
	}
}

pub fn clone(a: &[i32], b: &mut [i32]) {
	if a.len() != b.len() {
		panic!("Array lengths must be equal");
	}

	for i in 0..a.len() {
		b[i] = a[i];
	}
}

pub fn print_slice(a: &[i32], i: usize, j: usize) {
	print!("[");
	for k in i..j-1 {
		print!("{}, ", a[k]);
	}
	println!("{}]", a[j-1]);
}

pub fn println_slice(a: &[i32], i: usize, j: usize) {
	print!("[");
	for k in i..j-1 {
		println!("{}, ", a[k]);
	}
	println!("{}]", a[j-1]);
}

pub fn shuffle(a: &mut [i32]) {
	let mut rng = rand::thread_rng();

	for i in 0..a.len()-1 {
		let j = rng.gen_range(0, (i+1) as u32);
		swap(a, i as usize, j as usize);
	}
}

pub fn swap(a: &mut [i32], i: usize, j: usize) {
	let tmp = a[i];
	a[i] = a[j];
	a[j] = tmp;
}