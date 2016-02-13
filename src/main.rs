extern crate rand;
extern crate time;

mod sort;

use rand::Rng;
use time::*;

fn main() {
	let size = 100000;
	let mut a: [isize; 100000] = [0; 100000];
	let len = a.len();

	fill_random(&mut a, len);

	println!("Start sorting");
	let start = time::precise_time_ns();

	sort::sort_and_switch(&mut a, 0, (len-1) as isize, size);

	let duration = ((time::precise_time_ns() - start) as f64) / ((1000 * 60) as f64);

	println!("Finished sorting in {} s", duration);

	let is_sorted = check_sorted(&a);
	println!("Is array sorted? {}", is_sorted);
}

fn check_sorted(a: &[isize]) -> bool {
	let mut ret = true;
	for i in 1..a.len() {
		if a[i] < a[i-1] {
			ret = false;
		}
	}

	ret		// Return this value
}

fn fill_random(a: &mut [isize], len: usize) {
	let mut rng = rand::thread_rng();

	for i in 0..len {
		a[i] = rng.gen::<isize>();
	}
}

// fn fill_sorted(a: &mut [isize], len: usize) {
// 	for i in 0..len {
// 		a[i] = rng.gen::<i8>();
// 	}
// }
