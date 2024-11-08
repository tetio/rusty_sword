mod model;
mod service;


use model::heroes::Hero;
use model::monsters::Monster;
use model::weapons::*;

fn main() {
    let arn = Hero::builder()
        .name("Arn".to_string())
        .add_item(make_sword())
        .add_item(make_bow())
        .add_hp(12)
        .armour(12)
        .build();
    //.add_weapon(Weapon::Sword(6));
    //heroe.add_weapon(Weapon::Dagger(4));
    let goblin = Monster::builder()
        .name("Goblin".to_string())
        .hp(5)
        .thac0(19)
        .weapon(make_dagger())
        .build();
    println!("{:?}", arn);
    println!("{:?}", goblin);
}
