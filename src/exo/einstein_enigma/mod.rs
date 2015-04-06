#[derive(Debug)]
enum Couleur{
	Rouge,
	Verte,
	Blanche,
	Jaune,
	Bleue
}

#[derive(Debug)]
enum Nationalite{
	Anglais,
	Suedois,
	Danois,
	Norvegien,
	Allemand
}

#[derive(Debug)]
enum Boisson{
	The,
	Cafe,
	Lait,
	Biere,
	Eau
}

#[derive(Debug)]
enum Cigarette{
	PallMall,
	Dunhills,
	Blends,
	BleueMasters,
	Prince,
}

#[derive(Debug)]
enum Animaux{
	Chien,
	Oiseaux,
	Chats,
	Chevaux,
	PoissonRouge
}

#[derive(Debug)]
struct Maison{
	couleur : Couleur,
	nationalite : Nationalite,
	boisson : Boisson,
	cigarette : Cigarette,
	animaux : Animaux
}

fn maison_hypothese_origin() -> Maison{
	let m= Maison { couleur : Couleur::Rouge, nationalite : Nationalite::Anglais, boisson : Boisson::The, cigarette : Cigarette::PallMall, animaux : Animaux::Chien };
	m
}



pub fn launch()->(){
	println!("Einstein enigma program");
	let o=maison_hypothese_origin() ;
	println!("{:?}",o);
}