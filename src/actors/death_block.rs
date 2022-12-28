use std::collections::hash_map::RandomState;

use crate::render;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct DeathBlock {
    pub pos_x: usize,
    pub pos_y: usize,
    pub color: usize,
    pub width: usize,
    pub height: usize,
    pub speed: usize,
    pub speed_update_needed: bool,
    pub version: DeathBlockVersion,
    pub invisible: bool,
}

pub enum DeathBlockVersion {
    Normal,
    Big,
    Smoll,
    Fly,
    Long
}

pub fn initialize_death_block() -> DeathBlock {
    return DeathBlock {
        pos_x: 600,
        pos_y: 200,
        width: 40,
        height: 40,
        speed: 4,
        color: 0x267326,
        speed_update_needed: false,
        version: DeathBlockVersion::Big,
        invisible: false,
    };
}

pub fn update_death_block_state(death_block: &mut DeathBlock) {
    //Update position
    if death_block.pos_x > 40 {
        death_block.pos_x -= death_block.speed;
    } else {
        //Update speed
        if death_block.speed_update_needed {
            death_block.speed += 1;
            death_block.speed_update_needed = false;
        }
        //Reset position
        death_block.pos_x = 600;

        //Update size
        let new_version = [
            DeathBlockVersion::Normal,
            DeathBlockVersion::Big,
            DeathBlockVersion::Smoll,
            DeathBlockVersion::Fly,
            DeathBlockVersion::Long
        ]
        .choose(&mut rand::thread_rng())
        .unwrap();

        match new_version {
            DeathBlockVersion::Big => {
                death_block.version = DeathBlockVersion::Big;
                death_block.width = 50;
                death_block.height = 50;
                death_block.pos_y = 190;
            }
            DeathBlockVersion::Smoll => {
                death_block.version = DeathBlockVersion::Smoll;
                death_block.width = 30;
                death_block.height = 30;
                death_block.pos_y = 210;
            }
            DeathBlockVersion::Fly => {
                death_block.version = DeathBlockVersion::Fly;
                death_block.width = 40;
                death_block.height = 40;
                death_block.pos_y = 170;
            }
            DeathBlockVersion::Normal => {
                death_block.version = DeathBlockVersion::Normal;
                death_block.width = 40;
                death_block.height = 40;
                death_block.pos_y = 200;
            },
            DeathBlockVersion::Long => {
                death_block.version = DeathBlockVersion::Long;
                death_block.width = 60;
                death_block.height = 40;
                death_block.pos_y = 200;
            }
        }
    }
}

pub fn draw_death_block(buffer: &mut Vec<u32>, death_block: &DeathBlock) {
    if death_block.invisible {
        return;
    }

    render::draw_nonrectangle(
        buffer,
        &death_block.pos_x,
        &death_block.pos_y,
        &death_block.width,
        &death_block.height,
        death_block.color
    );
}
