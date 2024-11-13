use crate::model::mobs::Mob;
use crate::model::monsters::MonsterQuality;
use crate::model::{self, monsters};

use model::heroes::Hero;
use model::monsters::Monster;
use model::weapons::*;

use rand::Rng;

const STEPS: i32 = 1000;
const HERO_CASUALTIES_THRESHOLD: usize = 66;
const MONSTER_CASUALTIES_THRESHOLD: usize = 50;

pub fn hero_attacks(monster: Monster, hero: &Hero) -> Monster {
    //-> Mob {
    let attack_value = hero.mob.thac0;
    let defense_value = monster.mob.armour;
    let damage: i32 = if is_attack_successful(attack_value, defense_value) {
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
    let damage: i32 = if is_attack_successful(attack_value, defense_value) {
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

fn is_attack_successful(attack_value: i32, defense_value: i32) -> bool {
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

pub fn single_combat(hero: &mut Hero, monster: &mut Monster) -> (i32, String) {
    let mut rounds: i32 = 0;
    while hero.mob.hp > 0 && monster.mob.hp > 0 {
        match rounds % 2 {
            0 => attack(&hero.mob, &mut monster.mob),
            1 => attack(&monster.mob, &mut hero.mob),
            _ => panic!("Invalid round number"),
        }
        rounds += 1;
    }
    if hero.mob.hp <= 0 {
        (rounds, "monster".to_string())
    } else {
        (rounds, "hero".to_string())
    }
}

pub fn battle(human_army: &mut Vec<Hero>, monster_army: &mut Vec<Monster>) -> (i32, i32) {
    let human_army_size = human_army.len();
    let monster_army_size = monster_army.len();
    let mut end_of_battle = false;
    let mut total_rounds = 0;
    while !end_of_battle {
        let res = single_combat(&mut human_army[0], &mut monster_army[0]);
        total_rounds += res.0;
        if res.1.as_str() == "hero" {
            monster_army.remove(0);
        } else if res.1.as_str() == "monster" {
            human_army.remove(0);
        } else {
            panic!("Invalid winner");
        }
        end_of_battle = human_army.len() < human_army_size * HERO_CASUALTIES_THRESHOLD / 100
            || monster_army.len() < monster_army_size * MONSTER_CASUALTIES_THRESHOLD / 100;
    }
    println!(
        "Human casualties = {}, Monster casualties = {}",
        human_army_size - human_army.len(),
        monster_army_size - monster_army.len()
    );
    println!("Total rounds {}", total_rounds);
    let who_wins = match (
        human_army.len() >= human_army_size * HERO_CASUALTIES_THRESHOLD / 100,
        monster_army.len() >= monster_army_size * MONSTER_CASUALTIES_THRESHOLD / 100,
    ) {
        (true, false) => 1,
        (false, true) => -1,
        (_, _) => 0,
    };
    (total_rounds, who_wins)
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
            .level(1)
            .build();
        let monster = Monster::builder()
            .name("Goblin".to_string())
            .hp(MOB_HP)
            .weapon(make_dagger())
            .thac0(19)
            .armour(9)
            .monster_quality(MonsterQuality::Elite)
            .level(1)
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

    #[test]
    fn battle_0_test() {
        let hero = Hero::builder()
            .name("Arn".to_string())
            .weapon(make_sword())
            .thac0(18)
            .armour(8)
            .level(1)
            .hp_per_level("1d8".to_string())
            .build();
        let monster = Monster::builder()
            .name("Goblin".to_string())
            .weapon(make_dagger())
            .thac0(19)
            .armour(9)
            .level(1)
            .hp_per_level("1d6".to_string())
            .monster_quality(MonsterQuality::Elite)
            .build();

        let mut human_army = (0..1000).map(|_| hero.clone()).collect::<Vec<Hero>>();
        let mut monster_army = (0..2000).map(|_| monster.clone()).collect::<Vec<Monster>>();
        let res = battle(&mut human_army, &mut monster_army);

        if res.1 == 0 {
            println!("Draw");
        } else if res.1 == 1 {
            println!("Humans win");
        } else {
            println!("Monsters win");
        }
        assert!(res.0 > 1);
    }
}
