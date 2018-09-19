/***************************************************
* Nicholas L. Robson
* CIS 400 Rust Programming
* Homework 3
* Professor Royer
****************************************************/

use std::cmp;
use std::f32;

fn main() {
    let initial = [66, 70, 52, 93, 44, 67, 47, 10, 11, 13, 94, 99, 51, 12];
    let mut to_sort;
    println!("initial:          {:?}",initial);

    to_sort  = initial.clone();
    bubble_sort(&mut to_sort);
    println!("bubble-sorted:    {:?}",to_sort);

    to_sort  = initial.clone();
    sel_sort(&mut to_sort);
    println!("selection-sorted: {:?}",to_sort);

    to_sort  = initial.clone();
    insert_sort(&mut to_sort);
    println!("insertion-sorted: {:?}",to_sort);

    println!();
    println!("unordered search:");
    report_search(44,unordered_search(44,&initial[..]));
    report_search(43,unordered_search(43,&initial[..]));

    println!();
    println!("binary search:");
    report_search(44,binary_search(44,&to_sort[..]));
    report_search(43,binary_search(43,&to_sort[..]));

    println!("Sorted List: {:?}",to_sort);

    println!();
    println!("the min and max of initial are {:?}",
             min_max(&initial[..]));
}

/*
// NOTE!! The following will not compile: It needs lifetime annotations.
// We'll fix this later on.
fn swap(x :&mut u32, y : &mut u32) {
    let t = x;
    x = y;
    y = t;
}
*/

fn bubble_sort(a : &mut [u32]) {
    let len = a.len();
    for i in 0..len {
        for j in 0..(len-i-1) {
            if a[j]>a[j+1] {
                // swap the values of a[j] and a[j+1]
                let t = a[j];
                a[j] = a[j+1];
                a[j+1] = t;
            }
        }
    }
}

fn report_search(x : u32, r : Option<usize>) {
    print!("\t {} ",x);
    match r {
        None    => { println!("not found"); },
        Some(i) => { println!("found at index {}",i); },
    }
}

fn unordered_search(x : u32, a : &[u32]) -> Option<usize> {
    for i in 0..a.len() {
        if x==a[i] { return Some(i); }
    }
    None
}


fn sel_sort(a : &mut [u32]) {
	let len = a.len();
	for i in 0..len {
		let mut j = i + 1;
		let mut iMin = i;
		for j in (i)..(len){
			if a[j] < a[iMin] {
				iMin = j;
			}
		}

		if iMin != i {
			let t = a[i];
			a[i] = a[iMin];
			a[iMin] = t;
		}
	}
}

fn insert_sort(a : &mut [u32]) {
	let len = a.len();
	for i in 0..len {
		for j in (0..i).rev() {
			if a[j] >= a[j+1] {
				let t = a[j];
				a[j]=a[j+1];
				a[j+1] = t;
			}
		}
	}

}

fn binary_search(x : u32, a : &[u32]) -> Option<usize> {
	let len = a.len();
	let mut left = 0;
	let mut right = len-1;

	while left <= right {
		let  m_temp = ((left + right) / 2 ) as f32;
		let m = m_temp.floor() as usize;
		if a[m] < x {
			left = m + 1;
		}
		else if a[m] > x {
			right = m - 1;
		}
		else if a[m] == x{
			return Some(m)
		}
	}
    None
}

fn min_max(a : &[u32]) -> (u32,u32) {
    let len = a.len();
    assert!(len>0);
    if len == 1 {
    	return (a[0],a[0]);
    }
    //else if len == 2{
    //	return (cmp::min(a[0],a[1]) , cmp::max(a[0],a[1]));
    //}
    else{
    let m_temp = (len/2) as f32;
    let m = m_temp.floor() as usize;

	println!("Left-min/max: {:?}",&a[0..m]);
    let (lMin, lMax) = min_max(&a[0..m]);
    println!("Right-min/max: {:?}",&a[m..len]);
    let (rMin, rMax) = min_max(&a[m..len]);

    return ( cmp::min(lMin,rMin) , cmp::max(lMax,rMax) );
	}
}
    
// NOTE:
// cmp::min(a,b) returns the minimum of a and b
// cmp::max(a,b) returns the maximum of a and b

