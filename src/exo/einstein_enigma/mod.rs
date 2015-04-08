use std::fmt;


#[derive(Debug,Clone)]
enum Couleur{
	Rouge,
	Verte,
	Blanche,
	Jaune,
	Bleue
}
impl Couleur{
	fn inc(self) -> Couleur{
		    match self{

	    	Couleur::Rouge =>{let r=Couleur::Verte; 
	    	r},
	    	Couleur::Verte =>{let r=Couleur::Blanche;
	    	 r},
	    	Couleur::Blanche =>{let r=Couleur::Jaune; 
	    	r},
	    	Couleur::Jaune =>{let r=Couleur::Bleue;
	    	 r},
	    	Couleur::Bleue =>Couleur::Rouge
	    }
	}
}

#[derive(Debug,Clone)]
enum Nationalite{
	Anglais,
	Suedois,
	Danois,
	Norvegien,
	Allemand
}
impl Nationalite{
	fn inc(self) -> Nationalite{
		    match self{

	    	Nationalite::Anglais =>{let r=Nationalite::Suedois; 
	    	r},
	    	Nationalite::Suedois =>{let r=Nationalite::Danois;
	    	 r},
	    	Nationalite::Danois =>{let r=Nationalite::Norvegien; 
	    	r},
	    	Nationalite::Norvegien =>{let r=Nationalite::Allemand;
	    	 r},
	    	Nationalite::Allemand =>Nationalite::Anglais
	    }
	}
}

#[derive(Debug,Clone)]
enum Boisson{
	The,
	Cafe,
	Lait,
	Biere,
	Eau
}

impl Boisson{
	fn inc(self) -> Boisson{
		    match self{

	    	Boisson::The =>{let r=Boisson::Cafe; 
	    	r},
	    	Boisson::Cafe =>{let r=Boisson::Lait;
	    	 r},
	    	Boisson::Lait =>{let r=Boisson::Biere; 
	    	r},
	    	Boisson::Biere =>{let r=Boisson::Eau;
	    	 r},
	    	Boisson::Eau =>Boisson::The
	    }
	}
}

#[derive(Debug,Clone)]
enum Cigarette{
	PallMall,
	Dunhills,
	Blends,
	BleueMasters,
	Prince,
}
impl Cigarette{
	fn inc(self) -> Cigarette{
		    match self{

	    	Cigarette::PallMall =>{let r=Cigarette::Dunhills; 
	    	r},
	    	Cigarette::Dunhills =>{let r=Cigarette::Blends;
	    	 r},
	    	Cigarette::Blends =>{let r=Cigarette::BleueMasters; 
	    	r},
	    	Cigarette::BleueMasters =>{let r=Cigarette::Prince;
	    	 r},
	    	Cigarette::Prince =>Cigarette::PallMall
	    }
	}
}

#[derive(Debug,Clone)]
enum Animaux{
	Chien,
	Oiseaux,
	Chats,
	Chevaux,
	PoissonRouge
}
impl Animaux{
	fn inc(self) -> Animaux{
		    match self{

	    	Animaux::Chien =>{let r=Animaux::Oiseaux; 
	    	r},
	    	Animaux::Oiseaux =>{let r=Animaux::Chats;
	    	 r},
	    	Animaux::Chats =>{let r=Animaux::Chevaux; 
	    	r},
	    	Animaux::Chevaux =>{let r=Animaux::PoissonRouge;
	    	 r},
	    	Animaux::PoissonRouge =>Animaux::Chien
	    }
	}
}

#[derive(Debug,Clone)]
struct Maison{
	couleur : Couleur,
	nationalite : Nationalite,
	boisson : Boisson,
	cigarette : Cigarette,
	animaux : Animaux
}




impl Maison{


	fn inc(self) -> Maison{
	

		    match self{


		    	Maison {couleur : Couleur::Bleue,
		    	 nationalite : Nationalite::Allemand,
		    	  boisson : Boisson::Eau,
		    	   cigarette : Cigarette::Prince,
		    	    animaux : Animaux::PoissonRouge
		    	    } =>
		    	{
		    	 	Maison{couleur : Couleur::Bleue.inc(), nationalite : Nationalite::Allemand.inc(), boisson : Boisson::Eau.inc(), cigarette : Cigarette::Prince.inc(), animaux : Animaux::PoissonRouge.inc()}
		    	 },	

		    	Maison {couleur : Couleur::Bleue,
		    	 nationalite : Nationalite::Allemand,
		    	  boisson : Boisson::Eau,
		    	   cigarette : Cigarette::Prince,
		    	    animaux : e
		    	    } =>
		    	{
		    	 	Maison{couleur : Couleur::Bleue.inc(), nationalite : Nationalite::Allemand.inc(), boisson : Boisson::Eau.inc(), cigarette : Cigarette::Prince.inc(), animaux : e.inc()}
		    	 },		


		    	Maison {couleur : Couleur::Bleue,
		    	 nationalite : Nationalite::Allemand,
		    	  boisson : Boisson::Eau,
		    	   cigarette : d,
		    	    animaux : e
		    	    } =>
		    	{
		    	 	Maison{couleur : Couleur::Bleue.inc(), nationalite : Nationalite::Allemand.inc(), boisson : Boisson::Eau.inc(), cigarette : d.inc(), animaux : e}
		    	 },		    

		    	Maison {couleur : Couleur::Bleue,
		    	 nationalite : Nationalite::Allemand,
		    	  boisson : c,
		    	   cigarette : d,
		    	    animaux : e
		    	    } =>
		    	{
		    	 	Maison{couleur : Couleur::Bleue.inc(), nationalite : Nationalite::Allemand.inc(), boisson : c.inc(), cigarette : d, animaux : e}
		    	 },

		    	Maison {couleur : Couleur::Bleue,
		    	 nationalite : b,
		    	  boisson : c,
		    	   cigarette : d,
		    	    animaux : e
		    	    } =>
		    	{
		    	 	Maison{couleur : Couleur::Bleue.inc(), nationalite : b.inc(), boisson : c, cigarette : d, animaux : e}
		    	 },
		    	 

		    	Maison {couleur : a,
		    	 nationalite : b,
		    	  boisson : c,
		    	   cigarette : d,
		    	    animaux : e
		    	    } =>
		    	{
		    	 	Maison{couleur : a.inc(), nationalite : b, boisson : c, cigarette : d, animaux : e}
		    	 }
		   

	    }

	    
	} // fn inc maison

}	// impl Maison

	fn origin() -> Maison{
	let m= Maison { couleur : Couleur::Rouge, nationalite : Nationalite::Anglais, boisson : Boisson::The, cigarette : Cigarette::PallMall, animaux : Animaux::Chien };
	m
	}

/*impl Iterator for Maison {
type Item = Maison;
    fn next(&mut self) -> Option<Maison> {
        let n=(*self).inc();
        Some(n)
    }
}*/	

#[derive(Debug)]
struct HypotheseVector(Vec<Maison>,Vec<Maison>,Vec<Maison>,Vec<Maison>,Vec<Maison>);


fn full_possibility() -> HypotheseVector{
	let mut r=HypotheseVector(Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new());

	let mut o=origin() ;
	let nb_boucle=5*5*5*5*5 ;
	for _ in 0..nb_boucle{
		//println!("> {:?}", o);
		match r{
			HypotheseVector(ref mut a,ref mut b,ref mut c,ref mut d,ref mut e) => {
				a.push(o.clone());
				b.push(o.clone());
				c.push(o.clone());
				d.push(o.clone());
				e.push(o.clone());

			}

		}
		o=o.inc();

	}

	r
}

//1. L'Anglais vit dans la maison rouge.
		fn r1_anglais_rouge<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : Couleur::Rouge, nationalite : Nationalite::Anglais, boisson : _, cigarette : _, animaux : _} =>  true,
		    	Maison { couleur : _, nationalite : Nationalite::Anglais, boisson : _, cigarette : _, animaux : _} => false,
		    	Maison { couleur : Couleur::Rouge, nationalite : _, boisson : _, cigarette : _, animaux : _} =>  false,
		    	_ =>  true

		    }
		}

//2. Le Suédois élève des chiens.
		fn r2_suedois_chien<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : _, nationalite : Nationalite::Suedois, boisson : _, cigarette : _, animaux : Animaux::Chien} =>  true,
		    	Maison { couleur : _, nationalite : Nationalite::Suedois, boisson : _, cigarette : _, animaux :_} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : _, animaux : Animaux::Chien} =>  false,
		    	_ =>  true

		    }
		}		

//3. Le Danois boit du thé.
		fn r3_danois_the<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : _, nationalite : Nationalite::Danois, boisson : Boisson::The, cigarette : _, animaux : _} =>  true,
		    	Maison { couleur : _, nationalite : Nationalite::Danois, boisson : _, cigarette : _, animaux : _} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : Boisson::The, cigarette : _, animaux : _} =>  false,
		    	_ =>  true

		    }
		}			
//5. Le propriétaire de la maison verte boit du café.
		fn r5_verte_cafe<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : Couleur::Verte, nationalite : _, boisson : Boisson::Cafe, cigarette : _, animaux : _} =>  true,
		    	Maison { couleur : _, nationalite : _, boisson : Boisson::Cafe, cigarette : _, animaux : _} =>  false,
		    	Maison { couleur : Couleur::Verte, nationalite : _, boisson : _, cigarette : _, animaux : _}=>  false,
		    	_ =>  true

		    }
		}	

//6. Le fumeur de Pall Mall élève des oiseaux.
		fn r6_pallmall_oiseau<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : Cigarette::PallMall, animaux : Animaux::Oiseaux} =>  true,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : _, animaux : Animaux::Oiseaux} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : Cigarette::PallMall, animaux : _}=>  false,
		    	_ =>  true

		    }
		}		

//7. Le propriétaire de la maison jaune fume des Dunhills. 
		fn r7_jaune_dunhills<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : Couleur::Jaune, nationalite : _, boisson : _, cigarette : Cigarette::Dunhills, animaux : _} =>  true,
		    	Maison { couleur : Couleur::Jaune, nationalite : _, boisson : _, cigarette : _, animaux : _} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : Cigarette::Dunhills, animaux : _}=>  false,
		    	_ =>  true

		    }
		}				

//12. L'homme qui fume des Blue Masters boit de la bière.		
		fn r12_bluemaster_biere<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur :_  	, nationalite : _ 	, boisson : Boisson::Biere 	, cigarette : Cigarette::BleueMasters	, animaux : _} =>  true,
		    	Maison { couleur :_ 	, nationalite : _ 	, boisson : Boisson::Biere 	, cigarette : _ 						, animaux : _}  =>  false,
		    	Maison { couleur :_ 	, nationalite : _ 	, boisson : _				, cigarette : Cigarette::BleueMasters 	, animaux : _} =>  false,
		    	_ =>  true

		    }
		}	

//13. L'Allemand fume des Prince. 
		fn r13_allemand_prince<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur :_  	, nationalite : Nationalite::Allemand 	, boisson : _ 	, cigarette : Cigarette::Prince						, animaux : _} =>  true,
		    	Maison { couleur :_  	, nationalite : Nationalite::Allemand 	, boisson : _ 	, cigarette : _										, animaux : _}   =>  false,
		    	Maison { couleur :_  	, nationalite : _					 	, boisson : _ 	, cigarette : Cigarette::Prince						, animaux : _}  =>  false,
		    	_ =>  true

		    }
		}	

//1. L'Anglais vit dans la maison rouge.
//2. Le Suédois élève des chiens.
//3. Le Danois boit du thé.
//5. Le propriétaire de la maison verte boit du café.
//6. Le fumeur de Pall Mall élève des oiseaux.
//7. Le propriétaire de la maison jaune fume des Dunhills. 
//12. L'homme qui fume des Blue Masters boit de la bière.
//13. L'Allemand fume des Prince. 	

//4. La maison verte est juste à gauche de la maison blanche.
//8. L'homme qui vit dans la maison du centre boit du lait. 
//9. Le Norvégien vit dans la première maison. 
//10. L'homme qui fume des Blends vit à côté de celui qui élève des chats. 
//11. L'homme qui élève des chevaux vit à côté du fumeur de Dunhills. 
//14. Le Norvégien vit à côté de la maison bleue. 
//15. L'homme qui fume des Blends a un voisin qui boit de l'eau.	



		fn has_a_norvegien<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur :_  	, nationalite : Nationalite::Norvegien 	, boisson : _ 	, cigarette : _						, animaux : _				 } =>  true,
		    	_ =>  false

		    }
		}	
		fn has_lait<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur :_  	, nationalite : _ 	, boisson : Boisson::Lait 	, cigarette : _						, animaux : _				 } =>  true,
		    	_ =>  false

		    }
		}		
		fn has_bleue<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur :Couleur::Bleue  	, nationalite : _ 	, boisson : _ 	, cigarette : _						, animaux : _				 } =>  true,
		    	_ =>  false

		    }
		}				

		fn vec_has<F>(v : &Vec<Maison>,rule : F) -> bool
		where F: Fn(&Maison) -> bool{

			    for f in v.iter() {
			        if rule(&f) {return true}
			    }

			    false

		}

		fn vec_only<F>(v : &Vec<Maison>,rule : F) -> bool
		where F: Fn(&Maison) -> bool{

			    for f in v.iter() {
			        if !rule(&f) {return false}
			    }

			    true

		}		


fn vec_string(v : &Vec<Maison>)-> String{
	let mut r=String::new();

			    for f in v.iter() {
			    	let ot=format!("{:?}\n",f);
			        r.push_str(&ot);
			    }
	r
	
}

impl fmt::Display for HypotheseVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self{
		HypotheseVector(ref a,ref b,ref c,ref d,ref e) => {
		    	write!(f,"\n++++++Maison 1++++++\n {}\n++++++Maison 2++++++\n {}\n++++++Maison 3++++++\n {}\n++++++Maison 4++++++\n {}\n++++++Maison 5++++++\n {}",vec_string(a),vec_string (b),vec_string (c),vec_string (d), vec_string (e))

			}

		}
    }
}


impl HypotheseVector{

	fn count_determinant ( &mut self ) -> usize{
		match *self{
		HypotheseVector(ref mut a,ref mut b,ref mut c,ref mut d,ref mut e) => {
				a.len()+b.len()+c.len()+d.len()+e.len()

			}

		}


	}

	fn constraint_voisin_keep_not_in_or_voisin_in<F,G>(&mut self,rule1 : F,rule2 : G)
	where F: Fn(&Maison) -> bool, G: Fn(&Maison) -> bool{

		match *self{
		HypotheseVector(ref mut a,ref mut b,ref mut c,ref mut d,ref mut e) => {
				let ba=vec_has(&a,|x: &Maison| rule1(x));
				let bb=vec_has(&b,|x: &Maison| rule1(x));
				let bc=vec_has(&c,|x: &Maison| rule1(x));
				let bd=vec_has(&d,|x: &Maison| rule1(x));
				let be=vec_has(&e,|x: &Maison| rule1(x));

				a.retain(|x: &Maison| !rule2(x) || 		bb);
				b.retain(|x: &Maison| !rule2(x) || ba || 	bc);
				c.retain(|x: &Maison| !rule2(x) || bb || 	bd);
				d.retain(|x: &Maison| !rule2(x) || bc || 	be);
				e.retain(|x: &Maison| !rule2(x) || bd );
			}

		}

	}		

	fn constraint_voisin_keep_in_or_not_voisin_onlyin<F,G>(&mut self,rule1 : F,rule2 : G)
	where F: Fn(&Maison) -> bool, G: Fn(&Maison) -> bool{

		match *self{
		HypotheseVector(ref mut a,ref mut b,ref mut c,ref mut d,ref mut e) => {
				let ba=vec_only(&a,|x: &Maison| !rule1(x));
				let bb=vec_only(&b,|x: &Maison| !rule1(x));
				let bc=vec_only(&c,|x: &Maison| !rule1(x));
				let bd=vec_only(&d,|x: &Maison| !rule1(x));
				let be=vec_only(&e,|x: &Maison| !rule1(x));

				a.retain(|x: &Maison| rule2(x) || 		!bb);
				b.retain(|x: &Maison| rule2(x) || (!ba && 	!bc));
				c.retain(|x: &Maison| rule2(x) || (!bb && 	!bd));
				d.retain(|x: &Maison| rule2(x) || (!bc && 	!be));
				e.retain(|x: &Maison| rule2(x) || !bd );
			}

		}

	}	

	fn constraint_voisin<F,G>(&mut self,rule1 : F,rule2 : G)
	where F: Fn(&Maison) -> bool, G: Fn(&Maison) -> bool{


		self.constraint_voisin_keep_not_in_or_voisin_in(|x: &Maison| rule1(x),|x: &Maison| rule2(x));
		self.constraint_voisin_keep_not_in_or_voisin_in(|x: &Maison| rule2(x),|x: &Maison| rule1(x));

		self.constraint_voisin_keep_in_or_not_voisin_onlyin(|x: &Maison| rule1(x),|x: &Maison| rule2(x));
		self.constraint_voisin_keep_in_or_not_voisin_onlyin(|x: &Maison| rule2(x),|x: &Maison| rule1(x));


	}	

//4. La maison verte est juste à gauche de la maison blanche.
	fn r4_maison_verte_gauche_blanche<'cl, 'a> ( &mut self ) -> (){



	}

//8. L'homme qui vit dans la maison du centre boit du lait. 
	fn r8_centre_lait<'cl, 'a> ( &mut self ) -> (){
		match *self{
		HypotheseVector(ref mut a,ref mut b,ref mut c,ref mut d,ref mut e) => {
				a.retain(|x: &Maison| !has_lait(x));
				b.retain(|x: &Maison| !has_lait(x));
				c.retain(|x: &Maison| has_lait(x));
				d.retain(|x: &Maison| !has_lait(x));
				e.retain(|x: &Maison| !has_lait(x));

			}

		}


	}

//9. Le Norvégien vit dans la première maison. 	
	fn r9_norvegien_premiere<'cl, 'a> ( &mut self ) -> (){
		match *self{
		HypotheseVector(ref mut a,ref mut b,ref mut c,ref mut d,ref mut e) => {
				a.retain(|x: &Maison| has_a_norvegien(x));
				b.retain(|x: &Maison| !has_a_norvegien(x));
				c.retain(|x: &Maison| !has_a_norvegien(x));
				d.retain(|x: &Maison| !has_a_norvegien(x));
				e.retain(|x: &Maison| !has_a_norvegien(x));

			}

		}
	}
//14. Le Norvégien vit à côté de la maison bleue. 	
	fn r14_norvegien_cote_maison_bleue<'cl, 'a> ( &mut self ) -> (){
		self.constraint_voisin(|x: &Maison| has_a_norvegien(x),|x: &Maison| has_bleue(x));
	}

	fn apply<F>(&mut self,rule : F)
	where F: Fn(&Maison) -> bool{

		match *self{
		HypotheseVector(ref mut a,ref mut b,ref mut c,ref mut d,ref mut e) => {
				a.retain(|x: &Maison| rule(x));
				b.retain(|x: &Maison| rule(x));
				c.retain(|x: &Maison| rule(x));
				d.retain(|x: &Maison| rule(x));
				e.retain(|x: &Maison| rule(x));

			}

		}

	}

	fn simple_constraints(&mut self){
		self.apply(r1_anglais_rouge);
		self.apply(r2_suedois_chien);
		self.apply(r3_danois_the);
		self.apply(r5_verte_cafe);
		self.apply(r6_pallmall_oiseau);
		self.apply(r7_jaune_dunhills);
		self.apply(r12_bluemaster_biere);
		self.apply(r13_allemand_prince);


	}	


	fn other_constraintes(&mut self){
		self.r4_maison_verte_gauche_blanche();
		self.r8_centre_lait();
		self.r9_norvegien_premiere();
		self.r14_norvegien_cote_maison_bleue();
	}
}



pub fn launch()->(){

	println!("Einstein enigma program");


	let mut h=full_possibility();
	println!("> determinant de base : {} ",h.count_determinant());

	h.simple_constraints();
	//println!("> {}", h);

	//println!("> determinant apres contraintes simples : {} ",h.count_determinant());

	let mut count_apply=0;

	 loop{
		count_apply=count_apply+1;
		let prev_det=h.count_determinant();
		println!("-------------------------------");
		println!("-------------------------------");
		println!("-------------------------------");
		println!("-------------------------------");
		println!("-------------------------------");

		h.other_constraintes();
		println!("> {}", h);
		println!("> determinant : {} :  apres {} application des regles ",h.count_determinant(), count_apply);

		if prev_det==h.count_determinant() {break;}
	}



}


