// Not in the standard library, note that I'm pulling this package from the 
// web via crates.io, it needed to be listed in Cargo.toml dependecies
use rand::Rng;

// Can be used like classic enum:
enum Color {
	Red,
	Green,
	Blue,
}

enum Fruit {
	Apple,
	Orange,
	Banana,
	Grape,
}

// Can be used to solve the lack of null
fn maybe_get_color() -> Option<Color> {
	let chance = rand::thread_rng().gen_range(0..100);

	println!("Generated: {}", chance);

	// match is an expression
	match chance {
		0..=24 => Some(Color::Red), // Option::Some, but included by default
		25..=49 => Some(Color::Green),
		50..=74 => Some(Color::Blue),
		_ => None,    // Option::None, but included by default
	}
}

fn maybe_get_fruit() -> Option<Fruit> {
	let chance = rand::thread_rng().gen_range(0..120);

	println!("Generated: {}", chance);

	// match is an expression
	match chance {
		0..=24 => Some(Fruit::Apple), // Option::Some, but included by default
		25..=50 => Some(Fruit::Orange),
		51..=75 => Some(Fruit::Banana),
		76..=100 => Some(Fruit::Grape),
		_ => None,    // Option::None, but included by default
	}
}

fn main() {
	let maybe_col = maybe_get_color();
	match maybe_col {
		// Can provide a pattern to match and destructure the Option returned
		Some(c) => match c {
			Color::Red => println!("Got red!"),
			Color::Green => println!("Got green!"),
			Color::Blue => println!("Got blue!"),
		},
		None => println!("Got nothing :("),
	}

	let maybe_fruit = maybe_get_fruit();
	match maybe_fruit {
		// Can provide a pattern to match and destructure the Option returned
		Some(c) => match c {
			Fruit::Apple => println!("Got Apple!"),
			Fruit::Orange => println!("Got Orange!"),
			Fruit::Banana => println!("Got Banana!"),
			Fruit::Grape => println!("Got Grape!"),
		},
		None => println!("No fruit for you :("),
	}
}

//TODO: Write your own enum, and use it with another match statement

//<Your code here>
