#![allow(dead_code)]


fn invert (input : &str)-> String{
	let mut res=String::new();

	let chars: Vec<char> = input.chars().collect();

	let mut it = chars.into_iter().rev();
	loop {
	    match it.next() {
	        Some(x) => res.push(x),
	        None => break,
	    }
	}	

	return res;
}

pub fn launch() -> (){
println!("Hello, invert_tab!");
	let input = "Hello there";
	println!("{}",input);

	let output=invert(input).to_string();
	println!("{}",output);


}