
// Public functions

pub fn insertion_sort(a: &mut [isize]) {
	// println!("insertion_sort");

	for j in 1..a.len() {
		let key = a[j];
		let mut i = (j-1) as isize;
		
		while i>=0 && a[i as usize] > key {
			a[(i+1) as usize] = a[i as usize];
			i = i-1;
		}

		a[(i+1) as usize] = key;
	}
}

pub fn quicksort(a: &mut [isize], p: isize, r: isize) {
	// println!("quicksort {} {}", p, r);

	if p < r {
		let q = partition(a, p, r);
		quicksort(a, p, q-1);
		quicksort(a, q+1, r);
	}
}

pub fn sort_and_switch(a: &mut [isize], p: isize, r: isize, k: usize) {
	// println!("sort_and_switch {} {} {}", p, r, k);

	if (k as isize) < r {
		if p < r {
			let q = partition(a, p, r);
			sort_and_switch(a, p, q-1, k);
			sort_and_switch(a, q+1, r, k);
		}
	} else {
		insertion_sort(a);
	}
}


// Private functions

fn partition(a: &mut [isize], p: isize, r: isize) -> isize {
	// println!("partition {} {}", p, r);
	let x = a[r as usize];
	let mut i = p-1;

	for j in p..r {
		if a[j as usize] <= x {
			i += 1;
			swap(a, i as usize, j as usize);
		}
	}

	swap(a, (i+1) as usize, r as usize);

	i+1		// return this value
}

fn swap(a: &mut [isize], i: usize, j: usize) {
	let tmp = a[i];
	a[i] = a[j];
	a[j] = tmp;
}