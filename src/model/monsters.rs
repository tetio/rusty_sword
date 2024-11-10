use super::mobs::*;
use super::weapons::*;

#[derive(Debug, Default, Clone, Copy)]
pub enum MonsterQuality {
    Rookie = 0,
    #[default]
    Normal,
    Special,
    Elite,
}

#[derive(Default, Clone, Debug)]
pub struct Monster {
    pub mob: Mob,
    pub monster_quality: MonsterQuality,
}

impl Monster {
    pub fn builder() -> MonsterBuilder {
        MonsterBuilder::default()
    }
}

#[derive(Default, Clone, Debug)]
pub struct MonsterBuilder {
    pub mob: Mob,
    monster_quality: MonsterQuality,
}

impl MonsterBuilder {
    pub fn name(mut self, name: String) -> MonsterBuilder {
        self.mob.name = name;
        self
    }
    pub fn hp(mut self, hp: i32) -> MonsterBuilder {
        self.mob.hp = hp;
        self
    }
    pub fn weapon(mut self, weapon: Weapon) -> MonsterBuilder {
        self.mob.weapon = weapon;
        self
    }
    pub fn thac0(mut self, thac0: i32) -> MonsterBuilder {
        self.mob.thac0 = thac0;
        self
    }
    pub fn armour(mut self, armour: i32) -> MonsterBuilder {
        self.mob.armour = armour;
        self
    }
    pub fn monster_quality(mut self, monster_quality: MonsterQuality) -> MonsterBuilder {
        self.monster_quality = monster_quality;
        self
    }
    pub fn level(mut self, level: i32) -> MonsterBuilder {
        self.mob.level = level;
        self
    }
    pub fn hp_per_level(mut self, hp_per_level: String) -> MonsterBuilder {
        self.mob.hp_per_level = hp_per_level;
        self
    }
    pub fn build(mut self) -> Monster {
        self.mob.hp = MobBuilder::calculate_hp(self.mob.level, self.mob.hp_per_level.as_str());
        Monster {
            mob: self.mob,
            monster_quality: self.monster_quality,
        }
    }
}

#[test]
fn mosters_eq_test() {
    println!("mosters_eq_test");
    let mut monster0 = Monster::builder()
        .name("Goblin".to_string())
        .hp(6)
        .weapon(make_dagger())
        .thac0(0)
        .armour(20)
        .monster_quality(MonsterQuality::Normal)
        .build();
    monster0.mob.thac0 = 19;
    let monster = Monster::builder()
        .name("Goblin".to_string())
        .hp(6)
        .weapon(make_dagger())
        .thac0(19)
        .armour(20)
        .monster_quality(MonsterQuality::Normal)
        .build();
    assert_eq!(monster.mob.thac0, monster0.mob.thac0)
}
