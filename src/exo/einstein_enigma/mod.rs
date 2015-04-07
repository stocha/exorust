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
		fn anglais_rouge<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : Couleur::Rouge, nationalite : Nationalite::Anglais, boisson : _, cigarette : _, animaux : _} =>  true,
		    	Maison { couleur : _, nationalite : Nationalite::Anglais, boisson : _, cigarette : _, animaux : _} => false,
		    	Maison { couleur : Couleur::Rouge, nationalite : _, boisson : _, cigarette : _, animaux : _} =>  false,
		    	_ =>  true

		    }
		}

//2. Le Suédois élève des chiens.
		fn suedois_chien<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : _, nationalite : Nationalite::Suedois, boisson : _, cigarette : _, animaux : Animaux::Chien} =>  true,
		    	Maison { couleur : _, nationalite : Nationalite::Suedois, boisson : _, cigarette : _, animaux :_} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : _, animaux : Animaux::Chien} =>  false,
		    	_ =>  true

		    }
		}		

//3. Le Danois boit du thé.
		fn danois_the<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : _, nationalite : Nationalite::Danois, boisson : Boisson::The, cigarette : _, animaux : _} =>  true,
		    	Maison { couleur : _, nationalite : Nationalite::Danois, boisson : _, cigarette : _, animaux : _} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : Boisson::The, cigarette : _, animaux : _} =>  false,
		    	_ =>  true

		    }
		}			
//5. Le propriétaire de la maison verte boit du café.
		fn verte_cafe<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : Couleur::Verte, nationalite : _, boisson : Boisson::Cafe, cigarette : _, animaux : _} =>  true,
		    	Maison { couleur : _, nationalite : _, boisson : Boisson::Cafe, cigarette : _, animaux : _} =>  false,
		    	Maison { couleur : Couleur::Verte, nationalite : _, boisson : _, cigarette : _, animaux : _}=>  false,
		    	_ =>  true

		    }
		}	

//6. Le fumeur de Pall Mall élève des oiseaux.
		fn pallmall_oiseau<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : Cigarette::PallMall, animaux : Animaux::Oiseaux} =>  true,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : _, animaux : Animaux::Oiseaux} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : Cigarette::PallMall, animaux : _}=>  false,
		    	_ =>  true

		    }
		}		

//7. Le propriétaire de la maison jaune fume des Dunhills. 
		fn jaune_dunhills<'r> (input : &'r Maison ) -> bool {
		    match *input{
		    	Maison { couleur : Couleur::Jaune, nationalite : _, boisson : _, cigarette : Cigarette::Dunhills, animaux : _} =>  true,
		    	Maison { couleur : Couleur::Jaune, nationalite : _, boisson : _, cigarette : _, animaux : _} =>  false,
		    	Maison { couleur : _, nationalite : _, boisson : _, cigarette : Cigarette::Dunhills, animaux : _}=>  false,
		    	_ =>  true

		    }
		}				


impl HypotheseVector{


	fn r1<'cl, 'a> ( &mut self ) -> (){
		//L'Anglais vit dans la maison rouge.






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


	fn all_rules(&mut self){
		self.r1();
		self.apply(anglais_rouge);
		self.apply(suedois_chien);
		self.apply(danois_the);
		self.apply(verte_cafe);
		self.apply(pallmall_oiseau);
		self.apply(jaune_dunhills);


	}
}



pub fn launch()->(){

	println!("Einstein enigma program");


	let mut h=full_possibility();

	h.all_rules();

	println!("> {:?}", h);

}


