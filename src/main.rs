extern crate time;

mod array;
mod sort;

use std::env;
use time::*;

fn main() {
	const SIZE: usize = 50000;
	let mut print_pretty = true;
	let mut a: [i32; SIZE] = [0; SIZE];
	let mut b: [i32; SIZE] = [0; SIZE];

	if env::args().nth(2).unwrap() == "ugly" {
		print_pretty = false;
	}
	
	let first_arg = env::args().nth(1).unwrap();
	match first_arg.parse::<isize>().unwrap() {
		1 => {
			// Random distribution
			array::fill_random(&mut a);
			array::clone(& a, &mut b);
			run_tests(&mut a, & b, 0, 5, 300, print_pretty);
		},
		2 => {
			// Random equal distribution
			array::fill_equal(&mut a);
			array::clone(&a, &mut b);
			run_tests(&mut a, &b, 0, 100, 5000, print_pretty);
		},
		3 => {
			// Close-to-sorted distribution
			array::fill_almost_sorted(&mut a);
			array::clone(&a, &mut b);
			run_tests(&mut a, &b, 0, 100, SIZE, print_pretty);
		},
		_ => {
			panic!("Unknown argument: {}", first_arg);
		}
	}
}

fn run_tests(a: &mut [i32], b: & [i32], start: usize, 
	incr: usize, end: usize, print_pretty: bool) {

	let len = a.len();
	let mut k = start;

	let start = time::precise_time_ns();
	sort::quicksort(a, 0, (len-1) as i32);
	let duration = (time::precise_time_ns() - start) as f64;
	let dur_ms = duration / ((1000 * 1000) as f64);
	if print_pretty {
		println!("\nQuicksort baseline: {} ms\n", dur_ms);
	}

	array::clone(b, a);

	// TODO: print to CSV file
	if print_pretty {
		println!("{0: <8} | {1: <15} | {2: <10}", 
			"k", "t [ms]", "sorted");
		println!("---------------------------------------");
	} else {
		println!("k,t");
	}
	
	while k <= end {
		let start = time::precise_time_ns();

		sort::sort_and_switch(a, 0, (len-1) as i32, k);

		let duration = (time::precise_time_ns() - start) as f64;
		let dur_ms = duration / ((1000 * 1000) as f64);
		let is_sorted = array::check_sorted(&a);
		
		// TODO: print to CSV file
		if print_pretty {
			println!("{0: <8} | {1: <15} | {2: <10}", 
				k, dur_ms, is_sorted);
		} else {
			println!("{},{}", k, dur_ms);
		}

		k += incr;
		array::clone(b, a);
	}
}

