use crate::actors::{death_block, player};
use death_block::DeathBlock;
use player::{Player, PlayerState};

use minifb::{Key, Window};
use std::time::SystemTime;

pub fn update_tick(last_tick: &mut SystemTime, tick: &mut u64) -> bool {
    let duration = SystemTime::now().duration_since(*last_tick).expect("REEE");

    if duration.as_millis() >= 100 {
        *tick += 1;
        *last_tick = SystemTime::now();
        return true;
    }

    false
}

pub fn new_tick_triggers(
    tick: &u64,
    player: &mut Player,
    death_block: &mut DeathBlock,
    score: &mut usize,
) {
    if let PlayerState::Dieing = player.state {
    } else {
        if tick % 3 == 0 {
            *score += 1;
        }
    }

    if tick % 70 == 0 {
        death_block.speed_update_needed = true;
    }
}

pub fn process_input(
    window: &Window,
    player: &mut Player,
    death_block: &mut DeathBlock,
    score: &mut usize,
) {
    if window.is_key_down(Key::Space) {
        if let PlayerState::Existing = player.state {
            player.state = PlayerState::Jumping;
        }
    }

    if window.is_key_down(Key::R) {
        *player = player::initialize_player();
        *death_block = death_block::initialize_death_block();
        *score = 0;
    }
}

pub fn detect_collision(player: &mut Player, death_block: &mut DeathBlock) {
    if 
        is_point_in_rect(&death_block.pos_x, &death_block.pos_y, &death_block.width, &player.pos_x, &player.pos_y) || 
        is_point_in_rect(&death_block.pos_x, &death_block.pos_y, &death_block.width, &(&player.pos_x + &player.width), &player.pos_y) ||
        is_point_in_rect(&death_block.pos_x, &death_block.pos_y, &death_block.width, &player.pos_x, &(&player.pos_y + &player.width)) ||
        is_point_in_rect(&death_block.pos_x, &death_block.pos_y, &death_block.width, &(&player.pos_x + &player.width), &(&player.pos_y + &player.width))
    {
        player.state = PlayerState::Dieing;
        death_block.color = 0x000000;
    }
}


fn is_point_in_rect(rect_x: &usize, rect_y: &usize, width: &usize, point_x: &usize, point_y: &usize) -> bool{
    return rect_x <= point_x && rect_x + width >= *point_x &&
        rect_y <= point_y && rect_y + width >= *point_y;
}