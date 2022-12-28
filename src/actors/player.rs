use crate::{render, HEIGHT};

pub enum PlayerState {
    Jumping,
    Falling,
    Dieing,
    Spawning,
    Existing,
    CrouchingDown,
    CrouchingUp
}

pub struct Player {
    pub pos_x: usize,
    pub pos_y: usize,
    pub state: PlayerState,
    pub color: usize,
    pub width: usize,
    pub height: usize,
    pub invisible: bool
}

pub fn initialize_player() -> Player {
    return Player {
        pos_x: 100,
        pos_y: 100,
        width: 40,
        height: 40,
        state: PlayerState::Spawning,
        color: 0xFFFFFF,
        invisible: false
    };
}

pub fn update_player_state(player: &mut Player) {
    match player.state {
        PlayerState::Jumping => {
            if player.pos_y > 100 {
                player.pos_y -= get_player_speed((player.pos_y - 98) as i64) + 1;
            } else {
                player.state = PlayerState::Falling;
            }
        }
        PlayerState::Falling => {
            if player.pos_y < 200 {
                player.pos_y += get_player_speed((player.pos_y - 98) as i64) + 1;
            } else {
                player.state = PlayerState::Existing;
            }
        }
        PlayerState::Dieing => {
            if player.pos_y < HEIGHT - player.width - 6 {
                player.pos_y += 6;
            } else {
                player.invisible = true;
            }
        }
        PlayerState::Spawning => {
            if player.pos_y < 200 {
                player.pos_y += 4;
            } else {
                player.state = PlayerState::Existing;
            }
        },
        PlayerState::CrouchingDown => {
            if player.pos_y < 230 {
                player.pos_y += 1;
                player.height -= 1;
            } else {
                player.state = PlayerState::CrouchingUp;
            }
        }
        PlayerState::CrouchingUp => {
            if player.pos_y >= 200 {
                player.pos_y -= 1;
                player.height += 1;
            } else {
                player.state = PlayerState::Existing;
            }
        }
        _ => {}
    }
}

pub fn draw_player(buffer: &mut Vec<u32>, player: &Player) {
    if player.invisible {
        return;
    }
    render::draw_nonrectangle(
        buffer,
        &player.pos_x,
        &player.pos_y,
        &player.width,
        &player.height,
        player.color,
    );
}

pub fn get_player_speed(x: i64) -> usize {
    let speed: f64 = -0.001 * (x - 50).pow(2) as f64 + 4.0;
    return speed.round() as usize;
}
