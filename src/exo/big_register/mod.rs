

use std::fmt;
use std::ops::{BitXor, BitAnd,BitOr,Not};

    trait RegisterMinimal<T = Self> : BitXor + BitAnd +BitOr +Not + Copy{
		fn size(&self) -> usize;
		fn get(&self,index : usize) -> u64;
		fn set(self,index : usize,value : u64)->T ;


    }

#[derive(Debug,Clone,Copy)]
struct R1b {
	v : u64

}


impl fmt::Display for R1b {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.v{
			1 => write!(f,"X"),
			0 => write!(f,"O"),
			_ => panic!()
		}
    }
}

impl R1b{
	pub fn new() -> R1b{
		R1b{v : 0}
	}
}


impl RegisterMinimal for R1b{
	fn size(&self) -> usize{
		1
	}
		fn get(&self,index : usize) -> u64{
			match (index,self.v){
				(0,1) => 1,
				(0,0) => 0,
				_ => panic!()
			}

		}
		fn set(self,index : usize,value : u64)->R1b {
			let mut res=self;
			match (index,value){
				(0,0) => res.v=0,
				(0,1) => res.v=1,
				_ => panic!("Only 0 and 1 for a register set")
			}
			res
		}
}

impl BitXor for R1b {
    type Output = R1b;

    fn bitxor(self, _rhs: R1b) -> R1b {
        R1b{v : self.v^_rhs.v }
    }
}

impl BitAnd for R1b {
    type Output = R1b;

    fn bitand(self, _rhs: R1b) -> R1b {
        R1b{v:self.v&_rhs.v}
    }
}

impl BitOr for R1b {
    type Output = R1b;

    fn bitor(self, _rhs: R1b) -> R1b {
        R1b{v: self.v|_rhs.v}
    }
}

impl Not for R1b {
    type Output = R1b;

    fn not(self) -> R1b {
        R1b{v: !self.v & 1}
    }
}

#[test]
fn r1b_basic() {
	let mut x=R1b::new();
	x=x.set(0,0);
	assert!(format!("{}",x)=="O");
	x=x.set(0,1);
	assert!(format!("{}",x)=="X");

	assert!(x.get(0)==1);
	x=x.set(0,0);
	assert!(x.get(0)==0);
}

pub fn launch()->(){
	let x=R1b::new();
	println!("{}",x);
}