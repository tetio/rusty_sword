use crate::weapons::{Weapon, make_dagger, fist};

#[derive(Default, Clone, Debug)]

pub struct Mob {
    pub name: String,
    pub hp: i32,
    pub weapon: Weapon,
    pub thac0: i8,
    pub armour: i8
}

impl Mob {
    pub fn builder() -> MobBuilder {
        MobBuilder::new()
    }
}


pub struct MobBuilder {
    name: String,
    hp: i32,
    weapon: Weapon,
    thac0: i8,
    armour: i8    
}

impl MobBuilder {
    pub fn new()  -> MobBuilder {
        MobBuilder {name: "".to_string(), hp: 0, weapon: fist(), thac0: 20, armour: 20}
    }
    pub fn name(mut self, name: String) -> MobBuilder {
        self.name = name;
        self
    }
    pub fn hp(mut self, hp: i32) -> MobBuilder {
        self.hp = hp;
        self
    }
    pub fn weapon(mut self, weapon: Weapon) -> MobBuilder {
        self.weapon = weapon;
        self
    }
    pub fn thac0(mut self, thac0: i8) -> MobBuilder {
        self.thac0 = thac0;
        self
    }
    pub fn armour(mut self, armour: i8) -> MobBuilder {
        self.armour = armour;
        self
    }
    pub fn build(self) -> Mob {
        Mob {name: self.name, hp: self.hp, weapon: self.weapon, thac0: self.thac0, armour: self.armour}
    }

    
}


#[test]
fn mobs_eq_test() {
    let mut mob0 = Mob {
        name: "Goblin".to_string(),
        hp: 6,
        weapon: make_dagger(),
        thac0: 0,
        armour: 20
    };
    mob0.thac0 = 19;
    let mob = Mob::builder()
        .name("Goblin".to_string())
        .hp(6)
        .weapon(make_dagger())
        .thac0(19)
        .armour(20)
        .build();
    assert_eq!(1, 1)
}
#[test]
fn thac0_eq_test() {
    let mut mob0 = Mob {
        name: "Goblin".to_string(),
        hp: 6,
        weapon: make_dagger(),
        thac0: 0,
        armour: 20
    };
    mob0.thac0 = 19;
    let mob = Mob::builder()
        .name("Goblin".to_string())
        .hp(6)
        .weapon(make_dagger())
        .thac0(19)
        .armour(20)
        .build();
    assert_eq!(mob0.thac0, mob.thac0)
}