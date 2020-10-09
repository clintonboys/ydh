use rltk::{ RGB, Rltk };
use specs::prelude::*;

const MAPWIDTH : usize = 80;
const MAPHEIGHT : usize = 43;
const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;
const FIELD_HEIGHT : i32 = 8;
const FIELD_WIDTH : i32 = 4;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Fence, Farm, FieldTile
}

pub struct Field {
    pub x1 : i32,
    pub y1 : i32,
    pub w1 : i32,
    pub h1 : i32
}

impl Field {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Field {
	Field{x1:x, y1:y, w1:w, h1:h}
    }
}

#[derive(Default)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub fields : Vec<Field>,
    pub width : i32,
    pub height : i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub blocked : Vec<bool>
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
	(y as usize * self.width as usize) + x as usize
    }
    
    pub fn get_fences(&mut self) {
	for x in 0..80 {
	    let idx = self.xy_idx(x as i32, 0);
	    self.tiles[idx] = TileType::Fence;
	    let idx = self.xy_idx(x as i32, (MAPHEIGHT - 1) as i32);
            self.tiles[idx] = TileType::Fence;
	};
	for y in 0..43 {
	    let idx = self.xy_idx(0, y as i32);
	    self.tiles[idx] = TileType::Fence;
	    let idx = self.xy_idx((MAPWIDTH - 1) as i32, y as i32);
            self.tiles[idx] = TileType::Fence;
	};
    }

    pub fn get_fields(&mut self) -> Vec<Field> {
	let mut fields : Vec<Field> = Vec::new();
	for x in (2..(MAPWIDTH - FIELD_WIDTH as usize)).step_by((FIELD_WIDTH + 2) as usize) {
	    for y in (2..(MAPHEIGHT - FIELD_HEIGHT as usize)).step_by((FIELD_HEIGHT + 2) as usize) {
		let new_field = Field::new(x as i32, y as i32, FIELD_WIDTH, FIELD_HEIGHT);
		for x_ in new_field.x1..(new_field.x1 + FIELD_WIDTH) {
		    for y_ in new_field.y1..(new_field.y1 + FIELD_HEIGHT) {
			let idx = self.xy_idx(x_, y_);
			self.tiles[idx] = TileType::FieldTile;
	    }
	}
		fields.push(new_field);
	    }
	}	
	fields
    }

    pub fn new_map() -> Map {
    let mut map = Map{
	tiles : vec![TileType::Farm; MAPCOUNT],
	fields: Vec::new(),
	width: MAPWIDTH as i32,
	height: MAPHEIGHT as i32,
	revealed_tiles: vec![false; MAPCOUNT],
	visible_tiles: vec![false; MAPCOUNT],
	blocked: vec![false; MAPCOUNT]
	};

    map.get_fences();
    map.fields = map.get_fields();

    map
    }
}

pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let mut y = 0;
    let mut x = 0;
    for (_, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Farm => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Fence => {
                ctx.set(x, y, RGB::from_f32(0.4, 0.4, 0.6), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
	    TileType::FieldTile => {
		ctx.set(x, y, RGB::from_f32(1.0, 1.0, 0.8), RGB::from_f32(0., 0., 0.), rltk::to_cp437('_'));
		
	    }
        }

        // Move the coordinates
        x += 1;
        if x > (MAPWIDTH - 1) {
            x = 0;
            y += 1;
        }
    }
}
