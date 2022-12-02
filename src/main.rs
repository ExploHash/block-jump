extern crate minifb;

mod actors {
    pub mod death_block;
    pub mod player;
}

mod game;
mod render;

use minifb::{Key, Window, WindowOptions};
use std::time::SystemTime;
use std::usize;
use std::fs;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32>;

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    //initialize actors
    let mut player = actors::player::initialize_player();
    let mut death_block = actors::death_block::initialize_death_block();

    //Intialize stats
    let mut score: usize = 0;
    
    //Initialize highscore
    let highscore_string = fs::read_to_string("highscore.txt")
        .expect("Failed reading highscore file");
    let mut highscore = highscore_string.parse::<usize>().unwrap();

    //Timing
    let mut last_tick = SystemTime::now();
    let mut tick: u64 = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //Time calculations
        let new_tick = game::update_tick(&mut last_tick, &mut tick);
        if new_tick {
            game::new_tick_triggers(&tick, &mut player, &mut death_block, &mut score);
        }

        //Clear
        buffer = vec![0; WIDTH * HEIGHT];
        game::draw_gui(&mut buffer, &score, &highscore);
        //Process input
        game::process_input(&window, &mut player, &mut death_block, &mut score);
        //Update state
        actors::player::update_player_state(&mut player);
        actors::death_block::update_death_block_state(&mut death_block);
        //Draw
        actors::player::draw_player(&mut buffer, &player);
        actors::death_block::draw_death_block(&mut buffer, &death_block);
        //Detect collision
        game::detect_collision(&mut player, &mut death_block);
        //Update highscore
        game::update_highscore(&mut player, &score, &mut highscore);
        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
