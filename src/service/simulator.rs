use crate::model;

use model::heroes::Hero;
use model::monsters::*;
use model::weapons::*;

use super::combat::simulate;
use super::thac0::{get_thac04_hero, get_thac04_monster};

fn set_up() -> (Hero, Monster) {
    let hero_level = 1;
    let hero = Hero::builder()
        .name("Arn".to_string())
        .add_item(make_bow())
        .add_hp(20)
        .armour(20)
        .weapon(make_sword())
        .level(hero_level)
        .thac0(get_thac04_hero(hero_level))
        .build();

    let monster_level = 1;
    let monster_quality = MonsterQuality::Rookie;

    let goblin = Monster::builder()
        .name("Goblin".to_string())
        .hp(6)
        .level(monster_level.clone())
        .weapon(make_dagger())
        .armour(10)
        .monster_quality(monster_quality)
        .thac0(get_thac04_monster(monster_level, monster_quality))
        .build();

    (hero, goblin)
}

#[test]
fn scen_0000() {
    println!("mixa mixa");
    let specs: Vec<(i32, i32)> = vec![(1, 10), (10, 1), (1, 5), (2, 1), (3, 1), (4, 1), (5, 1)];
    //let mut result: Vec<((Hero, Monster), f64)> = Vec::new();
    let (mut hero, mut monster) = set_up();
    let mut mean = 0.;
    let _result = specs.iter().map(|(hero_level, monster_level)| {
        hero.mob.level = *hero_level;
        hero.mob.thac0 = get_thac04_hero(*hero_level);
        monster.mob.level = *monster_level;
        // TODO update hero's hp
        //monster.mob.hp = monster_level*6;
        monster.mob.hp = monster_level * 6;
        monster.mob.thac0 = get_thac04_monster(*monster_level, monster.monster_quality);
        // TODO update monster's hp
        mean = simulate(&hero.mob, &monster.mob);
        ((hero.clone(), monster.clone()), mean)
    }).collect::<Vec<((Hero, Monster), f64)>>();
}
