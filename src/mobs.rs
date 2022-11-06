#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub hp: i8,
    pub damage: i8,
    pub thac0: i8
}

impl Mob {
    fn builder() -> MobBuilder {
        MobBuilder::default()
    }
}

#[derive(Default)]
struct MobBuilder {
    pub name: String,
    pub hp: i8,
    pub damage: i8,
    pub thac0: i8    
}

impl MobBuilder {
    fn name(mut self, name: String) -> MobBuilder {
        self.name = name;
        self
    }
    fn hp(mut self, hp: i8) -> MobBuilder {
        self.hp = hp;
        self
    }
    fn damage(mut self, damage: i8) -> MobBuilder {
        self.damage = damage;
        self
    }
    fn thac0(mut self, thac0: i8) -> MobBuilder {
        self.thac0 = thac0;
        self
    }
    fn build(self) -> Mob {
        Mob {name: self.name, hp: self.hp, damage: self.damage, thac0: self.thac0}
    }
}


#[test]
fn mobs_eq_test() {
    let mut mob0 = Mob {
        name: "Goblin".to_string(),
        hp: 6,
        damage: 4,
        thac0: 0
    };
    mob0.thac0 = 19;
    let mob = Mob::builder()
        .name("Goblin".to_string())
        .hp(6)
        .damage(4)
        .thac0(19)
        .build();
    assert_eq!(mob0, mob)
}
#[test]
fn thac0_eq_test() {
    let mut mob0 = Mob {
        name: "Goblin".to_string(),
        hp: 6,
        damage: 4,
        thac0: 0
    };
    mob0.thac0 = 19;
    let mob = Mob::builder()
        .name("Goblin".to_string())
        .hp(6)
        .thac0(19)
        .build();
    assert_eq!(mob0.thac0, mob.thac0)
}