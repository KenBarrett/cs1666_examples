fn main() {
	let strings = vec![
		String::from("first string"),
		String::from("a"),
		String::from("b"),
		String::from("fourth string"),
		String::from("c"),
	];

	let big_strings: Vec<_> = strings.iter().filter(|x| x.len() > 5).collect();

	println!("Big strings:");
	for s in big_strings.iter() {
		println!("\t{}", s);
	}
	println!("big_strings[0]: {}\n", big_strings[0]);

	let mut little_strings: Vec<_> = strings
		.into_iter()
		.filter(|x| x.len() < 5)
		.collect();

	println!("Little strings:");
	for s in little_strings.iter_mut() {
		s.push('!');
		println!("\t{}", s);
	}
	println!("little_strings[0]: {}\n", little_strings[0]);

	println!("Little strings again:");
	// By default, for uses `.into_iter()`
	for s in little_strings {
		println!("\t{}", s);
	}
	// This will error
	//println!("little_strings[0]: {}", little_strings[0]);

	println!();
	let ints = vec![5, 10, 15, 20, 25, 30];
	for i in ints {
		println!("int: {}", i);
	}
	// Still moved, even though i types implement Copy, Vec<i32> does not
	//println!("first int again: {}", ints[0]);

	let ints = vec![5, 10, 37, 84, 22, 3, 8, 100, 50, 45, 12, 7, 63];

	//TODO: Create a vector of all of the odd values in ints using iterators

	//<Your code here>

	let odd_ints: Vec<_> = ints.iter().filter(|x| *x%2 != 0).collect();

	println!("Odd ints:");
	for i in odd_ints.iter() {
		println!("int: {}", i);
	}

	//TODO: Create a vector of double all of the even values in ints using iterators

	//<Your code here>
	let mut even_ints: Vec<_> = ints.iter().filter(|x| *x%2 == 0).map(|x| x*2).collect();

	println!("even ints:");
	for i in even_ints.iter() {
		println!("int: {}", i);
	}
}
