use crate::{monsters::MonsterQuality};

const MAX_THAC0: i32 = 20;

pub fn getThac04Hero(hero_level: i32) -> i32 {
    MAX_THAC0 - hero_level
}


pub fn getThac04Monster(monster_level: i32, monster_quality: MonsterQuality) -> i32 {
    match monster_quality {
        MonsterQuality::Rookie => MAX_THAC0 - monster_level + 3,
        MonsterQuality::Normal => MAX_THAC0 - monster_level + 1,
        MonsterQuality::Special => MAX_THAC0 - monster_level,
        MonsterQuality::Elite => MAX_THAC0 - monster_level - 1
    }
    
}