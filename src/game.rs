use crate::actors::{death_block, player};
use death_block::DeathBlock;
use player::{Player, PlayerState};

use crate::render;
use minifb::{Key, Window};
use std::time::SystemTime;

use std::fs;

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
    if is_point_in_rect(
        &death_block.pos_x,
        &death_block.pos_y,
        &death_block.width,
        &player.pos_x,
        &player.pos_y,
    ) || is_point_in_rect(
        &death_block.pos_x,
        &death_block.pos_y,
        &death_block.width,
        &(&player.pos_x + &player.width),
        &player.pos_y,
    ) || is_point_in_rect(
        &death_block.pos_x,
        &death_block.pos_y,
        &death_block.width,
        &player.pos_x,
        &(&player.pos_y + &player.width),
    ) || is_point_in_rect(
        &death_block.pos_x,
        &death_block.pos_y,
        &death_block.width,
        &(&player.pos_x + &player.width),
        &(&player.pos_y + &player.width),
    ) {
        player.state = PlayerState::Dieing;
        death_block.invisible = true;
    }
}

fn is_point_in_rect(
    rect_x: &usize,
    rect_y: &usize,
    width: &usize,
    point_x: &usize,
    point_y: &usize,
) -> bool {
    return rect_x <= point_x
        && rect_x + width >= *point_x
        && rect_y <= point_y
        && rect_y + width >= *point_y;
}

pub fn update_highscore(player: &mut Player, score: &usize, highscore: &mut usize) {
    if let PlayerState::Dieing = player.state {
        if score > &*highscore {
            fs::write("highscore.txt", score.to_string()).expect("Failed writing highscore file");
            *highscore = *score;
        }
    }
}

pub fn draw_gui(buffer: &mut Vec<u32>, score: &usize, highscore: &usize) {
    //Draw green grass
    render::draw_nonrectangle(buffer, &0, &240, &crate::WIDTH, &120, 0x61de45);
    //Draw blue sky
    render::draw_nonrectangle(buffer, &0, &0, &crate::WIDTH, &240, 0x87c6e8);
    //Draw Points
    render::render_text(
        buffer,
        &("Score: ".to_owned() + &score.to_string()),
        &50,
        &50,
        0x000,
    );

    //Draw highscore
    render::render_text(
        buffer,
        &("HighScore: ".to_owned() + &highscore.to_string()),
        &200,
        &50,
        0x000,
    );
}
