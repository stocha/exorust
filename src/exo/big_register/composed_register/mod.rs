use std::fmt;
use std::ops::{BitXor, BitAnd,BitOr,Not,Shl,Shr};
use exo::big_register::interfaces::{RegisterMinimal};


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ComposedRegister {
	v : u64

}


impl fmt::Display for ComposedRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.v{
			1 => write!(f,"X"),
			0 => write!(f,"O"),
			_ => panic!()
		}
    }
}

impl ComposedRegister{
	pub fn new() -> ComposedRegister{
		ComposedRegister{v : 0}
	}
}


impl RegisterMinimal for ComposedRegister{
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
		fn set(self,index : usize,value : u64)->ComposedRegister {
			let mut res=self;
			match (index,value){
				(0,0) => res.v=0,
				(0,1) => res.v=1,
				_ => panic!("Only 0 and 1 for a register set")
			}
			res
		}
}

impl BitXor for ComposedRegister {
    type Output = ComposedRegister;

    fn bitxor(self, _rhs: ComposedRegister) -> ComposedRegister {
        ComposedRegister{v : self.v^_rhs.v }
    }
}

impl BitAnd for ComposedRegister {
    type Output = ComposedRegister;

    fn bitand(self, _rhs: ComposedRegister) -> ComposedRegister {
        ComposedRegister{v:self.v&_rhs.v}
    }
}

impl BitOr for ComposedRegister {
    type Output = ComposedRegister;

    fn bitor(self, _rhs: ComposedRegister) -> ComposedRegister {
        ComposedRegister{v: self.v|_rhs.v}
    }
}

impl Not for ComposedRegister {
    type Output = ComposedRegister;

    fn not(self) -> ComposedRegister {
        ComposedRegister{v: !self.v & 1}
    }
}

impl Shl<usize> for ComposedRegister {
    type Output = ComposedRegister;

    fn shl(self, _rhs: usize) -> ComposedRegister {
        ComposedRegister{v: (self.v<<_rhs & 1)}
    }
}

impl Shr<usize> for ComposedRegister {
    type Output = ComposedRegister;

    fn shr(self, _rhs: usize) -> ComposedRegister {
        ComposedRegister{v: (self.v>>_rhs) & 1}
    }
}



#[cfg(test)]
mod tests{
	use super::*;
	use exo::big_register::interfaces::{RegisterMinimal};	

	#[test]
	fn composed_register_basic() {
		let mut x=ComposedRegister::new();
		x=x.set(0,0);
		assert!(format!("{}",x)=="O");
		x=x.set(0,1);
		assert!(format!("{}",x)=="X");

		assert!(x.get(0)==1);
		x=x.set(0,0);
		assert!(x.get(0)==0);

		let mut x=ComposedRegister::new();
		let mut o=ComposedRegister::new();

		x=x.set(0,1);
		o=o.set(0,0);

		assert!(o^o==o);
		assert!(o^x==x);
		assert!(x^o==x);
		assert!(x^x==o);

		assert!(o&o==o);
		assert!(o&x==o);
		assert!(x&o==o);
		assert!(x&x==x);	

		assert!(o|o==o);
		assert!(o|x==x);
		assert!(x|o==x);
		assert!(x|x==x);	

		assert!(!o==x);
		assert!(!x==o);	

		assert!(x<<1==o);
		assert!(x>>1==o);		
		assert!(o<<1==o);
		assert!(o<<1==o);	
	}
}