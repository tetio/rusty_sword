use super::weapons::*;
use rand::Rng;
use regex::Regex;

#[derive(Default, Clone, Debug)]
pub struct Mob {
    pub name: String,
    pub hp: i32,
    pub weapon: Weapon,
    pub thac0: i32,
    pub armour: i32,
    pub level: i32,
    pub hp_per_level: String, // "1d6+1"
}

impl Mob {
    pub fn builder() -> MobBuilder {
        MobBuilder::new()
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

pub fn hp_generator(dices: &str) -> i32 {
    // 1d6+1
    let re = Regex::new(r"(\d+)[d](\d+)[+]?(\d*)").unwrap();
    //let text = "1d6+1";
    for cap in re.captures_iter(dices) {
        println!(
            "#dices: {} #sides: {}  bonus: {}",
            &cap[1], &cap[2], &cap[3]
        );
    }
    // &cap[1] * &cap[2] + &cap[3]
    re.captures(dices)
        .map(|cap| {
            cap[1].parse::<i32>().unwrap_or(0)
                * rand::thread_rng().gen_range(1..=cap[2].parse::<i32>().unwrap_or(0))
                + cap[3].parse::<i32>().unwrap_or(0)
        })
        .unwrap_or(-1)
}

pub struct MobBuilder {
    name: String,
    hp: i32,
    weapon: Weapon,
    thac0: i32,
    armour: i32,
    level: i32,
    hp_per_level: String,
}

impl MobBuilder {
    pub fn new() -> MobBuilder {
        MobBuilder {
            name: "".to_string(),
            hp: 0,
            weapon: fist(),
            thac0: 20,
            armour: 20,
            level: 1,
            hp_per_level: "1d6".to_string(),
        }
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
    pub fn hp_per_level(mut self, life_exp: String) -> MobBuilder {
        self.hp_per_level = life_exp;
        self
    }

    pub fn calculate_hp(level: i32, hp_per_level: &str) -> i32 {
        (1..=level).map(|_| hp_generator(hp_per_level)).sum()
    }

    pub fn build(self) -> Mob {
        let hp = MobBuilder::calculate_hp(self.level, &self.hp_per_level);
        Mob {
            name: self.name,
            hp,
            weapon: self.weapon,
            thac0: self.thac0,
            armour: self.armour,
            level: self.level,
            hp_per_level: self.hp_per_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn thac0_eq_test() {
        let mob = Mob::builder()
            .name("Goblin".to_string())
            .weapon(make_dagger())
            .thac0(19)
            .armour(20)
            .level(1)
            .hp_per_level("1d8".to_string())
            .build();
        assert!(mob.hp >= 1 && mob.hp <= 8);
    }
}
