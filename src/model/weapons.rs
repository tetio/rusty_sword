
#[derive(Default, Clone, Debug)]
pub struct Weapon {
    pub name: String, 
    pub damage: u8, 
    pub velocity: WeaponVelocity
}


#[derive(Default, Clone, Debug)]
pub enum WeaponVelocity {
    #[default]
    Low = 0,
    Normal,
    Fast,
    VeryFast
}

// #[derive(Debug)]
// pub enum Weapons {
//     None(),
//     Dagger(Weapon),
//     Sword(Weapon),
//     Mace(Weapon),
//     Bow(Weapon),
//     CrossBow(Weapon),
//     TwoHandedSword(Weapon),
//     BastardSword(Weapon),
//     Flail(Weapon),
// }

pub fn fist() -> Weapon {
    Weapon {name: "Fist".to_string(), damage: 2, velocity: WeaponVelocity::Normal}
}
pub fn make_dagger() -> Weapon {
    Weapon {name: "Dagger".to_string(), damage: 4, velocity: WeaponVelocity::Fast}
}
pub fn make_flail() -> Weapon {
    Weapon {name: "Flail".to_string(), damage: 4, velocity: WeaponVelocity::Normal}
}
pub fn make_sword() -> Weapon {
    Weapon {name: "Sword".to_string(), damage: 8, velocity: WeaponVelocity::Fast}
}
pub fn make_bow() -> Weapon {
    Weapon {name: "Bow".to_string(), damage: 6, velocity: WeaponVelocity::Fast}
}
pub fn make_cross_bow() -> Weapon {
    Weapon {name: "Cross Bow".to_string(), damage: 6, velocity: WeaponVelocity::Normal}
}

// pub const DAGGER: Weapon = Weapon {name: "Dagger".to_string(), damage: 4, velocity: WeaponVelocity::Fast};
// pub static FLAIL: Weapon = Weapon {name: "Flail".to_string(), damage: 4, velocity: WeaponVelocity::Normal};
// pub static SWORD: Weapon = Weapon {name: "Sword".to_string(), damage: 8, velocity: WeaponVelocity::Fast};
// pub static BOW: Weapon = Weapon {name: "Bow".to_string(), damage: 6, velocity: WeaponVelocity::Fast};
// pub static CROSS_BOW: Weapon = Weapon {name: "Cross Bow".to_string(), damage: 6, velocity: WeaponVelocity::Normal};