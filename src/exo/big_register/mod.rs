use std::fmt;

    trait RegisterMinimal {
		fn size(&self) -> usize;
		fn get(&self,index : usize) -> u64;
		fn set(&mut self,index : usize,value : u64);
    }

#[derive(Debug,Copy)]
struct R1b{
	v : bool

}


impl fmt::Display for R1b {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.v{
			true => write!(f,"X"),
			false => write!(f,"O")
		}
    }
}

impl R1b{
	pub fn new() -> R1b{
		R1b{v : false}
	}
}

impl RegisterMinimal for R1b{
	fn size(&self) -> usize{
		1
	}
		fn get(&self,index : usize) -> u64{
			match self.v{
				true => 1,
				false => 0
			}

		}
		fn set(&mut self,index : usize,value : u64){
			match value{
				0 => self.v=false,
				1 => self.v=true,
				_ => panic!("Only 0 and 1 for a register set")
			}
		}
}

#[test]
fn R1bBasic() {
	let mut x=R1b::new();
	assert!(format!("{}",x)=="O");
	x.set(0,1);
	assert!(format!("{}",x)=="X");

	assert!(x.get(0)==1);
	x.set(0,0);
	assert!(x.get(0)==0);
}

pub fn launch()->(){

	let x=R1b{v : true};

	println!("Program big_register Program");

	println!("reg is {} {:?}", x,x) ;

	let x=R1b{v : false};

	println!("reg is {} {:?}", x,x) ;

}