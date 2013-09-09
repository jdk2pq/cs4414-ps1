use std::{os, float, uint};

fn main() {
	let args: ~[~str] =os::args();
	let mut sum = 0.0;
	let mut counter = 0.0;
	for uint::range(1,args.len()) |x| {
		match float::from_str(args[x]) {
			Some(num) => { 
				sum += num;
				counter += 1.0; 
			}, 
			None => { 
				println(fmt!("Bad input: %s",args[x])); 
			} 
		};
	}
	println(fmt!("Average: %f", sum / counter));
}