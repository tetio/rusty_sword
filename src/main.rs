mod heroes;
mod mobs;

use heroes::{Heroe, Weapon};

fn main() {
    let arn = Heroe::builder()
        .name(String::from("Arn"))
        .add_weapon(Weapon::Sword(6))
        .add_weapon(Weapon::Bow(5))
        .build();
    //.add_weapon(Weapon::Sword(6));
    //heroe.add_weapon(Weapon::Dagger(4));
    println!("{:?}", arn);
}
