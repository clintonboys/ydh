use rltk::{GameState, Rltk, RGB, Point};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod clock_system;
use clock_system::ClockSystem;
mod seed_sowing_system;
use seed_sowing_system::SeedSowingSystem;
mod plant_bearing_system;
use plant_bearing_system::PlantBearingSystem;
mod seed_system;
use seed_system::SeedSystem;
mod player;
use player::*;
mod gui;
mod gamelog;

#[derive(PartialEq, Copy, Clone)]
pub enum VendorMode { Buy, Sell }

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput,
		    PreRun,
		    ShowInventory,
		    PlayerTurn,
		    ShowMealPlan,
		    Ticking,
		    ShowSowSeed,
		    ShowHarvestPlant,
		    ShowVendor,
		    MainMenu { menu_selection : gui::MainMenuSelection },
		    SaveGame }

pub struct State {
    pub ecs: World
}

impl State {
    fn run_systems(&mut self) {
	let mut clock = ClockSystem{};
    clock.run_now(&self.ecs);
	let mut seeds = SeedSowingSystem{};
	seeds.run_now(&self.ecs);
	let mut plants = PlantBearingSystem{};
	plants.run_now(&self.ecs);
    let mut seed_clocks = SeedSystem{};
    seed_clocks.run_now(&self.ecs);
	self.ecs.maintain();
    }
}

impl GameState for State {
    /// Implement the rltk GameState trait for our State structure. 
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();        

        draw_map(&self.ecs, ctx);

        {
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();

            let data = (&positions, &renderables).join().collect::<Vec<_>>();
            for (pos, render) in data.iter() {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }

            gui::draw_ui(&self.ecs, ctx);
        }

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun |
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let seed_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToSowSeed>();
                        let player_entity = self.ecs.fetch::<Entity>();
                        if let Some(pos) = self.ecs.write_storage::<Position>().get_mut(*player_entity) {
                            intent.insert(*self.ecs.fetch::<Entity>(), WantsToSowSeed{ 
                                seed: seed_entity, x1: pos.x, y1: pos.y
                            });
                        }        
                        newrunstate = RunState::AwaitingInput;
                        }
                    }
                }
	    RunState::ShowMealPlan => {
		let result = gui::show_year_plan(self, ctx);
		match result.0 {
		    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    _ => {}		    
		}
	    }
	    _ => {}
	}
        let mut runwriter = self.ecs.write_resource::<RunState>();
        *runwriter = newrunstate;
    }               
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Yearlong Daily Harvest")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Seed>();
    gs.ecs.register::<Meal>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<PlayerStats>();
    gs.ecs.register::<GameClock>();
    gs.ecs.register::<SeedClock>();
    gs.ecs.register::<WantsToSowSeed>();
    gs.ecs.register::<InPlayerInventory>();
    
    let map : Map = Map::new_map();

    let player_entity = gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
    .with(Player{})
    .with(PlayerStats{ total_money: 0})
    .with(GameClock{ duration: 0 })
	.with(Name{name: "Player".to_string() })
        .build();

    for i in 0..4 {
        let seed_entity = gs.ecs
        .create_entity()
	    .with(Seed{ value: 1, time_to_maturation: 1, lifespan: 10, name: "Apple seed".to_string() })
	    .with(Name{name: "Apple seed".to_string() })
        .with(InPlayerInventory{})
        .with(SeedClock{ duration: i })
        .build();
    gs.ecs.insert(seed_entity);
    }

    for i in 0..4 {
        let seed_entity = gs.ecs
        .create_entity()
	    .with(Seed{ value: 2, time_to_maturation: 1, lifespan: 10, name: "Pear seed".to_string() })
	    .with(Name{name: "Pear seed".to_string() })
        .with(InPlayerInventory{})
        .with(SeedClock{ duration: i })
        .build();
    gs.ecs.insert(seed_entity);
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(40, 25));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to Yearlong Daily Harvest".to_string()] });
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["You inherited a farm! There's four apple seeds planted!".to_string()] });
    rltk::main_loop(context, gs)
}
