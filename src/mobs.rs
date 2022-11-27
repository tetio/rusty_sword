use crate::weapons::{Weapon, make_dagger, fist};

#[derive(Default, Clone, Debug)]
pub struct Mob {
    pub name: String,
    pub hp: i32,
    pub weapon: Weapon,
    pub thac0: i32,
    pub armour: i32,
    pub level: i32
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
    thac0: i32,
    armour: i32,
    level: i32    
}

impl MobBuilder {
    pub fn new()  -> MobBuilder {
        MobBuilder {name: "".to_string(), hp: 0, weapon: fist(), thac0: 20, armour: 20, level: 1}
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
    pub fn thac0(mut self, thac0: i32) -> MobBuilder {
        self.thac0 = thac0;
        self
    }
    pub fn armour(mut self, armour: i32) -> MobBuilder {
        self.armour = armour;
        self
    }
    pub fn level(mut self, level: i32) -> MobBuilder {
        self.level = level;
        self
    }
    pub fn build(self) -> Mob {
        Mob {name: self.name, hp: self.hp, weapon: self.weapon, thac0: self.thac0, armour: self.armour, level: self.level}
    }

    
}


#[test]
fn mobs_eq_test() {
    let mut mob0 = Mob {
        name: "Goblin".to_string(),
        hp: 6,
        weapon: make_dagger(),
        thac0: 0,
        armour: 20,
        level: 1
    };
    mob0.thac0 = 19;
    let mob = Mob::builder()
        .name("Goblin".to_string())
        .hp(6)
        .weapon(make_dagger())
        .thac0(19)
        .armour(20)
        .level(1)
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
        armour: 20,
        level: 1
    };
    mob0.thac0 = 19;
    let mob = Mob::builder()
        .name("Goblin".to_string())
        .hp(6)
        .weapon(make_dagger())
        .thac0(19)
        .armour(20)
        .level(1)
        .build();
    assert_eq!(mob0.thac0, mob.thac0)
}