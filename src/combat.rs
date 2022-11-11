use crate::{heroes::Hero, mobs::Mob, weapons::{make_dagger, make_sword}};
use rand::Rng;

pub fn hero_attacks(mob: &mut Mob, hero: &Hero) { //-> Mob {
    let attack_value = hero.mob.thac0;
    let defense_value = mob.armour;
    let damage: i32 = if is_successful(attack_value, defense_value) {
        roll_dice(hero.mob.weapon.damage)
    } else {
        0
    };
    //println!("Attak {}, defense: {}, damage: {}", attack_value, defense_value, damage);
    mob.hp -= damage;
    //mob
}


fn roll_dice(dice: u8) -> i32 {
    let dice_faces: i32 = dice as i32;
    rand::thread_rng().gen_range(1..=dice_faces)
}





fn is_successful(attack_value: i8, defense_value: i8) -> bool {
    // attack_value: 5        defense_value: 20 -> min roll -15
    // attack_value: 19        defense_value: 10 -> min roll 9
    // attack_value: 19        defense_value: 0 -> min roll 19
    // attack_value: 9        defense_value: -1 -> min roll 10
    let roll = roll_dice(20);
    roll >= (attack_value - defense_value) as i32
}


fn simulate(hero: &Hero, mob: &Mob) {
    let mut result: Vec<i32> = Vec::new();
    const STEPS: i32 = 10000;

    for i in 0..STEPS {
        let dummy_hero = hero.clone();
        let mut dummy_mob = mob.clone();
        let mut tries: i32 = 0;
        while dummy_mob.hp > 0 && tries < STEPS {
            hero_attacks(&mut dummy_mob, &dummy_hero);
            tries +=1;
        }
        result.push(tries);
    }
    let sum = result.iter().fold(0, |mut sum, &x| {sum += x; sum});
    let mean = f64::from(sum) / f64::from(STEPS);
    println!("Hero vanquished mob in {} attacks", mean);
}

#[test]
fn attack_test1() {
    const MOB_HP: i32 = 12;
    let hero = Hero::builder()
        .name("Arn".to_string())
        .weapon(make_sword())
        .add_hp(20)
        .thac0(12)
        .armour(20)
        .build();

    let mut mob = Mob::builder()
        .name("Goblin".to_string())
        .hp(MOB_HP)
        .weapon(make_dagger())
        .thac0(19)
        .armour(3)
        .build();

    let mob0 = mob.clone();

    simulate(&hero, &mob);



    hero_attacks(&mut mob, &hero);
    hero_attacks(&mut mob, &hero);
    hero_attacks(&mut mob, &hero);

    let hp = mob.hp;

    assert!(MOB_HP >= mob.hp);
    assert!(mob0.hp >= mob.hp);   
    assert!(hp <= MOB_HP); 
    
    println!("Mob hp {}", mob.hp);

}
