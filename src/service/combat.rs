use crate::model;
use crate::model::mobs::Mob;
use crate::model::monsters::MonsterQuality;

use model::heroes::Hero;
use model::monsters::Monster;
use model::weapons::*;

use rand::Rng;

const STEPS: i32 = 1000;

pub fn hero_attacks(monster: Monster, hero: &Hero) -> Monster {
    //-> Mob {
    let attack_value = hero.mob.thac0;
    let defense_value = monster.mob.armour;
    let damage: i32 = if is_successful(attack_value, defense_value) {
        roll_dice(hero.mob.weapon.damage)
    } else {
        0
    };
    let final_hp = monster.mob.hp - damage;
    let mob = Mob {
        hp: final_hp,
        ..monster.mob
    };
    //println!("Attak {}, defense: {}, damage: {}", attack_value, defense_value, damage);
    Monster { mob, ..monster }
    //mob
}

pub fn attack(attacker: &Mob, defender: &mut Mob) {
    //-> Mob {
    let attack_value = attacker.thac0;
    let weapon = &attacker.weapon;
    let defense_value = defender.armour;
    let damage: i32 = if is_successful(attack_value, defense_value) {
        roll_dice(weapon.damage)
    } else {
        0
    };
    // let final_hp = defender.hp - damage;
    // Mob {
    //     hp: final_hp,
    //     ..defender
    // }
    defender.hp -= damage;
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
    roll >= attack_value - defense_value
}

pub fn simulate_0(hero: &Hero, monster: &Monster) -> f64 {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..STEPS {
        let dummy_hero = hero.clone();
        let mut dummy_monster = monster.clone();
        let mut tries: i32 = 0;
        while dummy_monster.mob.hp > 0 && tries < STEPS {
            dummy_monster = hero_attacks(dummy_monster, &dummy_hero);
            tries += 1;
        }
        result.push(tries);
    }
    calculate_mean(result)
}

fn calculate_mean(results: Vec<i32>) -> f64 {
    let sum = results.iter().fold(0, |mut sum, x| {
        sum += x;
        sum
    });
    let mean = f64::from(sum) / f64::from(STEPS);
    println!("Hero defeats mob in {} attacks after {} tries", mean, STEPS);
    mean
}

pub fn simulate(attacker: &Mob, defender: &Mob) -> f64 {
    let result = (0..STEPS)
        .into_iter()
        .map(|_| {
            let mut dummy_d = defender.clone();
            let mut tries = 0;
            while dummy_d.is_alive() {
                attack(attacker, &mut dummy_d);
                tries += 1;
            }
            tries
        })
        .collect::<Vec<i32>>();
    calculate_mean(result)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            .monster_quality(MonsterQuality::Elite)
            .build();

        (hero, monster)
    }

    #[test]
    fn attack_test1() {
        let (hero, mut monster) = set_up();

        monster = hero_attacks(monster, &hero);
        monster = hero_attacks(monster, &hero);
        monster = hero_attacks(monster, &hero);

        assert!(monster.mob.hp >= monster.mob.hp);

        println!("Mob hp {}", monster.mob.hp);
    }
    #[test]
    fn simulate_combat() {
        let (hero, monster) = set_up();
        let mob0 = monster.clone();
        simulate(&hero.mob, &monster.mob);
        assert!(mob0.mob.hp >= monster.mob.hp);
    }
}
