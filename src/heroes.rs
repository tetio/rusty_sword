#[derive(Debug)]
pub enum Weapon {
    Dagger(u8),
    Sword(u8),
    Bow(u8),
    Mace(u8)   
}

#[derive(Debug)]
pub struct Heroe {
    pub name: String,
    pub weapons: Vec<Weapon>,
    pub hp: i8,
}

impl Heroe {
    pub fn builder() -> HeroeBuilder {
        HeroeBuilder::default()
    }
}
#[derive(Default)]
pub struct HeroeBuilder {
    name: String,
    weapons: Vec<Weapon>,
    hp: i8,
}

impl HeroeBuilder {
    // pub fn new() -> HeroeBuilder {
    //     let weapons: Vec<Weapon> = Vec::new();
    //     let name = "".to_string();
    //     let hp = 0;
    //     HeroeBuilder { name, weapons, hp }
    // }

    pub fn name(mut self, name: String) -> HeroeBuilder {
        self.name = name;
        self
    }

    pub fn add_weapon(mut self, weapon: Weapon) -> HeroeBuilder {
        self.weapons.push(weapon);
        self
    }

    pub fn add_hp(mut self, hp: i8) -> HeroeBuilder {
        self.hp += hp;
        self
    }

    pub fn build(self) -> Heroe {
        Heroe {name: self.name, weapons: self.weapons, hp: self.hp}
    }
}


#[test]
fn builder_test() {
    let mut heroe = Heroe::builder().name("Arn".to_string()).add_weapon(Weapon::Bow(4)).add_hp(20).build();
    heroe.hp -= 5;
    assert_eq!(1, heroe.weapons.len());
}

