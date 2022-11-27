use crate::{monsters::MonsterQuality, monsters::Monster, heroes::Hero, thac0::getThac04Hero, thac0::getThac04Monster, weapons::make_dagger, weapons::make_bow, combat::simulate};

fn setUp() -> (Hero, Monster){
    let hero_level = 1;
    let mut hero = Hero::builder()
    .name("Arn".to_string())
    .add_item(make_bow())
    .add_hp(20)
    .armour(20)
    .level(hero_level)
    .thac0(getThac04Hero(hero_level))
    .build();

    const monster_level: i32 = 1;
    const monster_quality: MonsterQuality = MonsterQuality::Normal;

    let mut goblin = Monster::builder()
    .name("Goblin".to_string())
    .hp(6)
    .level(monster_level.clone())
    .weapon(make_dagger())
    .armour(20)
    .monster_quality(monster_quality.clone())
    .thac0(getThac04Monster(monster_level, monster_quality))
    .build();

    (hero, goblin)
    
}

#[test]
fn scen_0000() {
    println!("mixa mixa");
    let (hero, monster) = setUp();
    simulate(&hero, &monster)
}