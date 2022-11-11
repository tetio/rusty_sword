mod heroes;
mod mobs;
mod combat;
mod weapons;

use heroes::Hero;    
use mobs::Mob;
use weapons::{make_sword, make_bow, make_dagger};
fn main() {
    let arn = Hero::builder()
        .name(String::from("Arn"))
        .add_item(make_sword())
        .add_item(make_bow())
        .add_hp(12)
        .armour(12)
        .build();
    //.add_weapon(Weapon::Sword(6));
    //heroe.add_weapon(Weapon::Dagger(4));
    let goblin = Mob::builder()
        .name("Goblin".to_string())
        .hp(5)
        .thac0(19)
        .weapon(make_dagger())
        .build();
    println!("{:?}", arn);
    println!("{:?}", goblin);
}
