use super::mobs::*;
use super::weapons::*;


#[derive(Debug, Clone)]
pub struct Hero {
    pub mob: Mob,
    pub inventory: Vec<Weapon>,

}

impl Hero {
    pub fn builder() -> HeroBuilder {
        HeroBuilder::default()
    }
}
#[derive(Default)]
pub struct HeroBuilder {
    mob: Mob,
    inventory: Vec<Weapon>,
}

impl HeroBuilder {
    // pub fn new() -> HeroeBuilder {
    //     let weapons: Vec<Weapon> = Vec::new();
    //     let name = "".to_string();
    //     let hp = 0;
    //     HeroeBuilder { name, weapons, hp }
    // }

    pub fn name(mut self, name: String) -> HeroBuilder {
        self.mob.name = name;
        self
    }

    pub fn add_item(mut self, weapon: Weapon) -> HeroBuilder {
        self.inventory.push(weapon);
        self
    }
    pub fn weapon(mut self, weapon: Weapon) -> HeroBuilder {
        self.mob.weapon = weapon;
        self
    }
    pub fn add_hp(mut self, hp: i32) -> HeroBuilder {
        self.mob.hp += hp;
        self
    }
    pub fn thac0(mut self, thac0: i32) -> HeroBuilder {
        self.mob.thac0 = thac0;
        self
    }
    pub fn armour(mut self, armour: i32) -> HeroBuilder {
        self.mob.armour = armour;
        self
    }
    pub fn level(mut self, level: i32) -> HeroBuilder {
        self.mob.level = level;
        self
    }
    pub fn life_exp(mut self, life_exp: String) -> HeroBuilder {
        self.mob.life_exp = life_exp;
        self
    }
    pub fn build(self) -> Hero {
        Hero {mob: self.mob, inventory: self.inventory}
    }
}


#[test]
fn builder_test() {
    let hp = (1..5).map(|_| calculate_hp("1d6+1")).sum(); //;
    let mut hero = Hero::builder()
        .name("Arn".to_string())
        .add_item(make_bow())
        .add_hp(hp)
        .thac0(16)
        .armour(20)
        .build();
    hero.mob.hp -= 5;
    assert_eq!(1, hero.inventory.len());
    assert_eq!(hp-5, hero.mob.hp)
}

