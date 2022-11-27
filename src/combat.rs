use crate::{heroes::Hero, mobs::Mob, weapons::{make_dagger, make_sword}, monsters::Monster};
use rand::Rng;

pub fn hero_attacks(monster: &mut Monster, hero: &Hero) { //-> Mob {
    let attack_value = hero.mob.thac0;
    let defense_value = monster.mob.armour;
    let damage: i32 = if is_successful(attack_value, defense_value) {
        roll_dice(hero.mob.weapon.damage)
    } else {
        0
    };
    //println!("Attak {}, defense: {}, damage: {}", attack_value, defense_value, damage);
    monster.mob.hp -= damage;
    //mob
}


fn roll_dice(dice: u8) -> i32 {
    let dice_faces: i32 = dice as i32;
    rand::thread_rng().gen_range(1..=dice_faces)
}





fn is_successful(attack_value: i32, defense_value: i32) -> bool {
    // attack_value: 5        defense_value: 20 -> min roll -15
    // attack_value: 19        defense_value: 10 -> min roll 9
    // attack_value: 19        defense_value: 0 -> min roll 19
    // attack_value: 9        defense_value: -1 -> min roll 10
    let roll = roll_dice(20);
    roll >= (attack_value - defense_value) as i32
}


pub fn simulate(hero: &Hero, mob: &Monster) {
    let mut result: Vec<i32> = Vec::new();
    const STEPS: i32 = 1000;

    for i in 0..STEPS {
        let dummy_hero = hero.clone();
        let mut dummy_monster = mob.clone();
        let mut tries: i32 = 0;
        while dummy_monster.mob.hp > 0 && tries < STEPS {
            hero_attacks(&mut dummy_monster, &dummy_hero);
            tries +=1;
        }
        result.push(tries);
    }
    let sum = result.iter().fold(0, |mut sum, x| {sum += x; sum});
    let mean = f64::from(sum) / f64::from(STEPS);
    println!("Hero vanquished mob in {} attacks after {} tries", mean, STEPS);
}



fn set_up() -> (Hero, Monster) {
    const MOB_HP: i32 = 12;
    let hero = Hero::builder()
        .name("Arn".to_string())
        .weapon(make_sword())
        .add_hp(20)
        .thac0(12)
        .armour(20)
        .build();
    let monster = Monster::builder()
        .name("Goblin".to_string())
        .hp(MOB_HP)
        .weapon(make_dagger())
        .thac0(19)
        .armour(9)
        .build();

        (hero, monster)
}
#[test]
fn attack_test1() {
    
    let (hero, mut monster) = set_up();

    let mob0 = monster.clone();

    hero_attacks(&mut monster, &hero);
    hero_attacks(&mut monster, &hero);
    hero_attacks(&mut monster, &hero);

    assert!(mob0.mob.hp >= monster.mob.hp);   
    
    println!("Mob hp {}", monster.mob.hp);

}

#[test]
fn sumulate_combat() {
    let (hero, monster) = set_up();

    let mob0 = monster.clone();

    simulate(&hero, &monster);

    assert!(mob0.mob.hp >= monster.mob.hp);  
}
