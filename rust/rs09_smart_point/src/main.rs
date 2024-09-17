//TODO: Use smart pointers to create a generic linked-list

//<Your code here>

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}

impl LinkedList<i32> {
    pub fn new() -> LinkedList<i32>{
        LinkedList {
            val: None,
            next: None,
        }
    }

	pub fn push_left(self, x: i32) -> LinkedList<i32> {
		let node= LinkedList::<i32> {
			val: Some(x),
			next: Some(Box::new(self))
		};
		node
	}
	
}

fn main() {
	let s10 = 10;
	let h10 = Box::new(10);

	println!("stack 10: {}", s10);
	println!("heap 10: {}", h10);
	// Will error, need to dereference smart pointer
	//println!("Equal? {}", s10 == h10);
	// OK
	println!("Equal? {}", s10 == *h10);

	println!();

	// Needed to do this with references as well!
	let s5 = 5;
	let r5 = &s5;
	println!("stack 5: {}", s5);
	println!("ref 5: {}", r5);
	// Will error, need to dereference the reference
	//println!("Equal? {}", s5 == r5);
	// OK
	println!("Equal? {}", s5 == *r5);

	// Edit and extend this example to learn more about how Box<T> and refs
	// Do you need to deref an &mut to assign a value to it?
	// Can you mutate a Box<T>?

	let mut list: LinkedList<i32> = LinkedList::<i32>::new();
	list = list.push_left(1);
	list = list.push_left(2);
	list = list.push_left(3);
	list.push_left(4);
}
