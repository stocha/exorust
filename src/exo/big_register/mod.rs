
pub mod interfaces;
pub mod r1b;

use exo::big_register::r1b::{R1b};



pub fn launch()->(){
	let x=R1b::new();
	println!("{}",x);
}