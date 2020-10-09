use specs::prelude::*;
use super::{RunState, gamelog::GameLog, GameClock, SeedClock};

pub struct SeedSystem {}

impl<'a> System<'a> for SeedSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        Entities<'a>,
                        WriteStorage<'a, SeedClock>,
                        ReadExpect<'a, Entity>, // The player
                        ReadExpect<'a, RunState>,
                        WriteExpect<'a, GameLog>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut seed_clock, player_entity, runstate, mut log) = data;

        for (entity, mut clock) in (&entities, &mut seed_clock).join() {
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
        }
    }
}
