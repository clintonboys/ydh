use specs::prelude::*;
use super::{RunState, gamelog::GameLog, GameClock, SeedClock, Seed, IsSown};

pub struct SeedSystem {}

impl<'a> System<'a> for SeedSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        Entities<'a>,
                        WriteStorage<'a, Seed>,
                        WriteStorage<'a, SeedClock>,
                        ReadExpect<'a, RunState>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, IsSown>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut seeds, mut clocks, runstate, mut log, is_sowns) = data;

        for (entity, mut seed, mut clock, is_sown) in (&entities, &seeds, &mut clocks, &is_sowns).join() {
            let mut proceed = true;

            match *runstate {
                RunState::PlayerTurn => proceed = true,
                _ => proceed = false
            }
            if proceed {
                clock.duration += 1;
            }
            let mut season_log = String::new();
            match clock.duration % seed.time_to_maturation {
                0 => season_log = format!("{} matured ({}, {})", seed.name,
            clock.duration, seed.time_to_maturation),
                _ => season_log = String::new()
            }
            if season_log.len() > 0 {
                log.entries.push(season_log); 
            }
        }
    }
}
