use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, State, TileType, Map, RunState};
use std::cmp::{max, min};

const MAPWIDTH : usize = 80;
const MAPHEIGHT : usize = 43;

fn xy_idx(x: i32, y: i32) -> usize {
	(y as usize * MAPWIDTH) + x as usize
    }

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Fence{
            pos.x = min((MAPWIDTH - 1) as i32, max(0, pos.x + delta_x));
            pos.y = min((MAPHEIGHT - 1) as i32, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::AwaitingInput } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::I => return RunState::ShowInventory,

            VirtualKeyCode::P => return RunState::ShowMealPlan,

            _ => { return RunState::AwaitingInput }
        },
    }
    RunState::PlayerTurn
}
