#![allow(dead_code)]
/*//use std::fmt;

use std::collections::hash_map::{self, HashMap};


#[derive( PartialEq,Eq,Debug,Clone,Hash)]
enum W{
	Couleur,
	Nationalite,
	Boisson,
	Animal,
	Cigarette,

	Rouge,
	Verte,
	Blanche,
	Jaune,
	Bleue,

	Anglais,
	Suedois,
	Danois,
	Norvegien,
	Allemand,

	The,
	Cafe,
	Lait,
	Biere,
	Eau,

	Chien,
	Oiseau,
	Chat,
	Cheval,
	Poisson,

	Blend,
	Dunhills,
	Pallmall,
	Bluemaster,
	Prince

}
struct WordModel{
	to_cat : HashMap<W,W>,
}

impl WordModel{

    fn new() -> WordModel {
        let mut r= WordModel { 
        	to_cat:HashMap::new()
        };
        r.declare_word_structure();
        r
    }

    pub fn get_cat(& self,val : &W) ->&W{
    	self.to_cat.get(val).unwrap()
    }

	pub fn decl( &mut self,cat : W, val : W){
		self.to_cat.insert(val,cat);
	}

	pub fn declare_word_structure( &mut self){
		self.decl(W::Couleur,W::Rouge);
		self.decl(W::Couleur,W::Verte);
		self.decl(W::Couleur,W::Blanche);
		self.decl(W::Couleur,W::Jaune);
		self.decl(W::Couleur,W::Bleue);

		self.decl(W::Nationalite,W::Anglais);
		self.decl(W::Nationalite,W::Suedois);
		self.decl(W::Nationalite,W::Danois);
		self.decl(W::Nationalite,W::Norvegien);
		self.decl(W::Nationalite,W::Allemand);	

		self.decl(W::Boisson,W::The);
		self.decl(W::Boisson,W::Cafe);
		self.decl(W::Boisson,W::Lait);
		self.decl(W::Boisson,W::Biere);
		self.decl(W::Boisson,W::Eau);		

		self.decl(W::Animal,W::Chien);
		self.decl(W::Animal,W::Oiseau);
		self.decl(W::Animal,W::Chat);
		self.decl(W::Animal,W::Cheval);
		self.decl(W::Animal,W::Poisson);	

		self.decl(W::Cigarette,W::Blend);
		self.decl(W::Cigarette,W::Dunhills);
		self.decl(W::Cigarette,W::Pallmall);
		self.decl(W::Cigarette,W::Bluemaster);
		self.decl(W::Cigarette,W::Prince);			
	}
	

}








pub fn launch()->(){

	println!("Einstein enigma program V2");

	let m=WordModel::new();
	println!("cat for {:?} is {:?}", W::Norvegien, m.get_cat(&W::Norvegien) );

}


*/