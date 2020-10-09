use counter::Counter;
use specs::prelude::*;
use rltk::{ RGB, Rltk, VirtualKeyCode };
use super::{Player, PlayerStats, gamelog::GameLog, Name, SeedClock, State, Seed, GameClock, Meal, InPlayerInventory};

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let player_stats = ecs.read_storage::<PlayerStats>();
    let players = ecs.read_storage::<Player>();
    let game_clock = ecs.read_storage::<GameClock>();
    for (_player, stats, clock) in (&players, &player_stats, &game_clock).join() {
        let health = format!(" Total money: {}", stats.total_money);
        let time_elapsed = format!("Days: {}", clock.duration);
        // let season = format!("{}", clock.season);
        ctx.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);
        ctx.print_color(40, 43, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &time_elapsed);
        //season_color = match season {
        //
        // }
        //ctx.print_color(40, 44, season_color, RGB::named(rltk::BLACK), &season);
    }

    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print(2, y, s); }
        y += 1;
    }

}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected }

pub fn show_inventory(gs : &mut State, ctx : &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let names = gs.ecs.read_storage::<Name>();
    let clocks = gs.ecs.read_storage::<SeedClock>();
    let player_inventory = gs.ecs.read_storage::<InPlayerInventory>();    
    let seeds = gs.ecs.read_storage::<Seed>();
    let entities = gs.ecs.entities();

    let inventory = (&names, &seeds, &player_inventory, &clocks).join();
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Inventory");
    ctx.print_color(18, y+count as i32+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESCAPE to cancel");

    let mut equippable : Vec<Entity> = Vec::new();
    let mut j = 0;
    let seed_counts = (&names, &player_inventory).join().collect::<Counter<_>>();
    let mut count_vec: Vec<_> = seed_counts.iter().collect();
    count_vec.sort_by(|a, b| (a.0).0.name.cmp(&(b.0).0.name));

    let mut seeds_for_selection = Vec::new();
    for ((name, in_inventory), freq) in count_vec.iter() {

        ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
        ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk ::BLACK), rltk::to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        // for (entity) in (name, &entities).join() {
        //     ctx.print(40+y, y, &name.name.to_string());
        // }
        ctx.print(40, y, &freq.to_string());
        let mut actual_seeds = Vec::new();
        let entity = (&seeds, &names, &player_inventory, &entities).join()
                .filter(|seed| seed.0.name == name.name )
                .for_each(|seed| {
                    actual_seeds.push(seed.3)});
        let num_seeds = actual_seeds.len();
        match num_seeds {
            0 => {},
            _ => seeds_for_selection.push(actual_seeds[0])
        };
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count_vec.len() as i32 {
                        // println!("{}", seeds_for_selection[selection as usize].name.to_string());
                        return (ItemMenuResult::Selected, Some(seeds_for_selection[selection as usize]));
                    }
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }
}

pub fn show_year_plan(gs : &mut State, ctx : &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let names = gs.ecs.read_storage::<Name>();
    let meals = gs.ecs.read_storage::<Meal>();
    let entities = gs.ecs.entities();

    let meals = (&names, &meals).join();

    ctx.draw_box(2, 2, 76, 39 as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(2, 2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Yearly meal plan");
    // ctx.print_color(18, y+count as i32+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESCAPE to cancel");

    //let mut equippable : Vec<Entity> = Vec::new();
    //let mut j = 0;
    //for (entity, name) in (&entities, &names).join() {
    //    ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
    //    ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
    //    ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

    //    ctx.print(21, y, &name.name.to_string());
    //    equippable.push(entity);
    //    y += 1;
    //    j += 1;
    //}

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => { (ItemMenuResult::NoResponse, None) }
            }
        }
    }
}
