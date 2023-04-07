use crate::prelude::*;
use bevy_asset_loader::prelude::*;

// use rltk::*;
use rltk::Point;
use rltk::BaseMap;
use rltk::DistanceAlg;


use smallvec::*;
use bevy::render::view::Visibility;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
    Void,
}

#[derive(Debug)]
pub struct Map {
    // for tiles, like wall, floor,...
    pub tiles: Vec<TileType>,
    // entities occupying the tiles, like player, enemies, objects, ...
    pub occupation: Vec<Option<Entity>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; NUM_TILES],
            occupation: vec![None; NUM_TILES],
        }
    }

    pub fn in_bounds<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        position.x >= 0 && position.x < SCREEN_WIDTH
            && position.y >= 0 && position.y < SCREEN_HEIGHT
    }

    // checks if it is physically possible (ie no wall or physical object)
    pub fn can_enter_tile<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position) && (
            self.tiles[map_idx(position.x, position.y)] == TileType::Floor ||
            self.tiles[map_idx(position.x, position.y)] == TileType::Exit
        )
    }

    // checks if another entity like an enemy or player, are already in that cell
    pub fn is_tile_occupied<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position)
            && self.occupation[map_idx(position.x, position.y)] == None
    }

    pub fn try_idx(&self, position: Position) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

use rltk::Algorithm2D;
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
    fn in_bounds(&self, item: Point) -> bool {
        self.in_bounds(item)
    }
}


impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> 
    {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1), self.index_to_point2d(idx2)
        )
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}


const TILESIZE: i32 = 16;
const POS_SPRITE: rltk::Point = rltk::Point{x:432/TILESIZE as i32, y:288/TILESIZE as i32};

pub fn spawn_map_tiles(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: Res<TextureAssets>,
) {
    println!("{:?}", mb.map.tiles);
    let texture_handle: Handle<Image> = textures.tilemap.clone();

    let texture_atlas = 
    TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(
            TILESIZE as f32,
            TILESIZE as f32
        ),
        POS_SPRITE.x as usize,
        POS_SPRITE.y as usize,
        None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            
            let idx = map_idx(x, y);
                match mb.map.tiles[idx] 
                {
                    TileType::Floor => 
                    {
                        commands
                        .spawn(
                        SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            sprite: TextureAtlasSprite::new((1*POS_SPRITE.x +1)  as usize),
                            transform: Transform::from_translation(Vec3::new(100., 0., 1.)),
                            ..default()
                        },

                        )
                        .insert(MapTile)
                        .insert(Position { x: x, y: y, z: 1 })
                        .insert(TileSize::square(1.0));

                    }
                    
                    tile_type @ (TileType::Wall | TileType::Exit) =>
                    {
                            commands
                            .spawn((
                                MapTile,
                                TileSize::square(1.0),
                                Position { x, y, z: 0 },
                                SpriteSheetBundle {
                                    texture_atlas: texture_atlas_handle.clone(),
                                    sprite: TextureAtlasSprite::new((10*POS_SPRITE.x +22)  as usize),
                                    transform: Transform::from_translation(Vec3::new(100., 0., 1.)),
                                    ..default()
                                },
                            ));
                    }
                    TileType::Void => ()
                }
        }
    }
}