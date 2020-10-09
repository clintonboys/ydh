use specs::prelude::*;
use super::{RunState, gamelog::GameLog, GameClock};

pub struct ClockSystem {}

impl<'a> System<'a> for ClockSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        Entities<'a>,
                        WriteStorage<'a, GameClock>,
                        ReadExpect<'a, Entity>, // The player
                        ReadExpect<'a, RunState>,
                        WriteExpect<'a, GameLog>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut game_clock, player_entity, runstate, mut log) = data;

        for (entity, mut clock) in (&entities, &mut game_clock).join() {
            let mut proceed = true;

            match *runstate {
                RunState::PlayerTurn => {
                    if entity == *player_entity {
                        proceed = true;
                    }
                }
                _ => proceed = false
            }

            if proceed {
                clock.duration += 1;
            }

            let mut season_log = String::new();
            match clock.duration % 400 {
                0 => season_log = String::from("Winter"),
                100 => season_log = String::from("Spring"),
                200 => season_log = String::from("Summer"),
                300 => season_log = String::from("Autumn"),
                _ => season_log = String::new()
            }
            match clock.duration % 100 {
                0 => log.entries.push(season_log),
                _ => ()
            }
        }
    }
}
