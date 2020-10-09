use rltk::{Rltk, RGB};
use specs::prelude::*;
use super::{RunState, gamelog::GameLog, GameClock, WantsToSowSeed, Seed, 
    Position, Renderable, InPlayerInventory, Name, IsSown};

pub struct SeedSowingSystem {}

impl<'a> System<'a> for SeedSowingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        Entities<'a>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Renderable>,
                        WriteStorage<'a, WantsToSowSeed>,
                        WriteStorage<'a, Seed>,
                        WriteStorage<'a, InPlayerInventory>,
                        WriteStorage<'a, IsSown>,
                        WriteExpect<'a, GameLog>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut positions, names, mut renderables, mut wants_sow, mut seed, mut inventory, mut is_sown, mut log) = data;
	for (entity, sow) in (&entities, &wants_sow).join() {
	    // Remove seed from player inventory
        inventory.remove(sow.seed);
        is_sown.insert(sow.seed, IsSown{});
        // Add seed to sown seeds
        let newx = sow.x1;
        let newy = sow.y1;
        positions.insert(sow.seed, Position{ x: newx, y: newy});
        let seed_name = names.get(sow.seed).unwrap();
        match &seed_name.name.to_owned()[..] {
            "Apple seed" => renderables.insert(sow.seed, Renderable{
                glyph: rltk::to_cp437('a'),
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
            }),
            "Pear seed" => renderables.insert(sow.seed, Renderable{
                glyph: rltk::to_cp437('p'),
                fg: RGB::from_f32(0.82, 0.886, 0.192),
                bg: RGB::named(rltk::BLACK),
            }),
            _ => {
                log.entries.push(seed_name.name.to_string());
                renderables.insert(sow.seed, Renderable{
                glyph: rltk::to_cp437('_'),
                fg: RGB::named(rltk::BLACK),
                bg: RGB::named(rltk::BLACK),
            })}
        };
	    let mut log_entry = String::from("Seed sown!");
	    log.entries.push(log_entry);
	}
	wants_sow.clear();
    }
}
