use crate::prelude::*;
// use rltk::*;
// use rltk::Point;
// use rltk::BaseMap;
// use rltk::DistanceAlg;


// use smallvec::*;

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
            self.tiles[map_idx(position.x, position.y)] == TileType::Exit ||
            self.tiles[map_idx(position.x, position.y)] == TileType::Wall
            
        ) 

    }



}





pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}


const TILESIZE: i32 = 16;
const POS_SPRITE: Point = Point{x:432/TILESIZE as i32, y:288/TILESIZE as i32};

pub fn spawn_map_tiles(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: Res<TextureAssets>,
) {
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
            
            commands
            .spawn(
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new((1*POS_SPRITE.x +1)  as usize),
                transform: Transform::from_translation(Vec3::new(100., 0., 0.3)),
                ..default()
            },
            )
            .insert(MapTile)
            .insert(Position { x: x, y: y, z: 0 })
            .insert(TileSize::square(1.0));

            let idx = map_idx(x, y);
                match mb.map.tiles[idx] 
                {
                    TileType::Floor => (),
                    TileType::Wall =>
                    {
                            commands
                            .spawn((
                                MapTile,
                                TileSize::square(1.0),
                                Position { x, y, z: 0 },
                                SpriteSheetBundle {
                                    texture_atlas: texture_atlas_handle.clone(),
                                    sprite: TextureAtlasSprite::new((10*POS_SPRITE.x +22)  as usize),
                                    ..default()
                                },
                            ));
                    }
                    TileType::Exit => (),
                    TileType::Void => ()
                }
        }
    }
}