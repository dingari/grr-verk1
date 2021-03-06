use super::array;

pub fn insertion_sort(a: &mut [i32]) {
	for j in 1..a.len() {
		let key = a[j];
		let mut i = (j-1) as i32;
		
		while i>=0 && a[i as usize] > key {
			a[(i+1) as usize] = a[i as usize];
			i = i-1;
		}

		a[(i+1) as usize] = key;
	}
}

pub fn quicksort(a: &mut [i32], p: i32, r: i32) {
	if p < r {
		let q = partition(a, p, r);
		quicksort(a, p, q-1);
		quicksort(a, q+1, r);
	}
}

pub fn sort_and_switch(a: &mut [i32], p: i32, r: i32, k: usize) {

	if (k as i32) < (r - p) {
		if p < r {
			let q = partition(a, p, r);
			sort_and_switch(a, p, q-1, k);
			sort_and_switch(a, q+1, r, k);
		}
	} else {
		insertion_sort(&mut a[(p as usize)..((r+1) as usize)]);
	}
}

fn partition(a: &mut [i32], p: i32, r: i32) -> i32 {
	let x = a[r as usize];
	let mut i = p-1;

	for j in p..r {
		if a[j as usize] <= x {
			i += 1;
			array::swap(a, i as usize, j as usize);
		}
	}

	array::swap(a, (i+1) as usize, r as usize);

	i+1		// return this value
}