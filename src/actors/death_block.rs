use crate::render;

pub struct DeathBlock {
    pub pos_x: usize,
    pub pos_y: usize,
    pub color: usize,
    pub width: usize,
    pub speed: usize,
    pub speed_update_needed: bool,
}

pub fn initialize_death_block() -> DeathBlock {
    return DeathBlock {
        pos_x: 600,
        pos_y: 200,
        width: 40,
        speed: 4,
        color: 0xFF0000,
        speed_update_needed: false,
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

        death_block.pos_x = 600;
    }
}

pub fn draw_death_block(buffer: &mut Vec<u32>, death_block: &DeathBlock) {
    render::draw_rectangle(
        buffer,
        &death_block.pos_x,
        &death_block.pos_y,
        &death_block.width,
        death_block.color,
    );
}
