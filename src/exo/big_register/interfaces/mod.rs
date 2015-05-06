//use std::fmt;
use std::ops::{BitXor, BitAnd,BitOr,Not,Shl,Shr};


    pub trait RegisterMinimal<T = Self> : BitXor + BitAnd +BitOr +Not + Shl<usize> + Shr<usize> + Copy{
		fn size(&self) -> usize;
		fn get(&self,index : usize) -> u64;
		fn set(self,index : usize,value : u64)->T ;


    }