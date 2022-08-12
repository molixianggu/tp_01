use std::io::BufReader;

use bevy::{
    asset::{AssetLoader, AssetPath, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    transform::TransformBundle,
    utils::HashMap,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::{Collider, Vect};

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(TilemapPlugin)
            .add_asset::<TiledMap>()
            .add_asset_loader(TiledLoader)
            .add_system(process_loaded_maps);
    }
}

#[derive(TypeUuid)]
#[uuid = "7b4b2475-9a8d-4676-b8b2-06639ff9511a"]
pub struct TiledMap {
    pub map: tiled::Map,
    pub tilesets: HashMap<String, Handle<Image>>,
}

// Stores a list of tiled layers.
#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
    pub storage: TiledLayersStorage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub struct TiledLoader;

impl AssetLoader for TiledLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let root_dir = load_context.path().parent().unwrap();

            let mut loader = tiled::Loader::new();
            let map = loader.load_tmx_map_from(BufReader::new(bytes), root_dir)?;

            // let map = tiled::parse(BufReader::new(bytes))?;

            let mut dependencies = Vec::new();
            let mut handles = HashMap::default();

            for tileset in map.tilesets() {
                let image_path = match &tileset.image {
                    Some(img) => img,
                    None => continue,
                };
                let tile_path = root_dir.join(image_path.source.clone());

                info!("load file: {:?}", tile_path);

                let asset_path = AssetPath::new(tile_path, None);
                let texture: Handle<Image> = load_context.get_handle(asset_path.clone());

                // for i in tileset.first_gid..(tileset.first_gid + tileset.tilecount.unwrap_or(1)) {

                // }
                handles.insert(tileset.name.clone(), texture.clone());

                info!("load file: {:?}", handles);

                dependencies.push(asset_path);
            }

            let loaded_asset = LoadedAsset::new(TiledMap {
                map,
                tilesets: handles,
            });
            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut map_query: Query<(&Handle<TiledMap>, &mut TiledLayersStorage)>,
    new_maps: Query<&Handle<TiledMap>, Added<Handle<TiledMap>>>,
) {
    let mut changed_maps = Vec::<Handle<TiledMap>>::default();
    for event in map_events.iter() {
        info!("load event: {:?}", event);
        match event {
            AssetEvent::Created { handle } => {
                info!("Map added!");
                changed_maps.push(handle.clone());
            }
            AssetEvent::Modified { handle } => {
                info!("Map changed!");
                changed_maps.push(handle.clone());
            }
            AssetEvent::Removed { handle } => {
                info!("Map removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps = changed_maps
                    .into_iter()
                    .filter(|changed_handle| changed_handle == handle)
                    .collect();
            }
        }
    }

    // If we have new map entities add them to the changed_maps list.
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.clone_weak());
    }

    for changed_map in changed_maps.iter() {
        for (map_handle, mut layer_storage) in map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle != changed_map {
                continue;
            }
            if let Some(tiled_map) = maps.get(map_handle) {
                // for (name, img) in tiled_map.tilesets.iter() {
                //     info!("name: {}", name);
                //     commands.spawn_bundle(SpriteBundle {
                //         texture: img.clone(),
                //         ..default()
                //     });
                // }

                // TODO: Create a RemoveMap component..
                for layer_entity in layer_storage.storage.values() {
                    if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                        for tile in layer_tile_storage.iter() {
                            if let Some(tile) = tile {
                                commands.entity(*tile).despawn_recursive()
                            }
                        }
                    }
                    // commands.entity(*layer_entity).despawn_recursive();
                }

                for tileset in tiled_map.map.tilesets() {
                    // Once materials have been created/added we need to then create the layers.
                    for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                        let tile_size = TilemapTileSize {
                            x: tileset.tile_width as f32,
                            y: tileset.tile_height as f32,
                        };
                        let tile_spacing = TilemapSpacing {
                            x: tileset.spacing as f32,
                            y: tileset.spacing as f32,
                        };

                        let offset_x = layer.offset_x;
                        let offset_y = layer.offset_y;

                        let map_size = TilemapSize {
                            x: tiled_map.map.width,
                            y: tiled_map.map.height,
                        };

                        let grid_size = TilemapGridSize {
                            x: tiled_map.map.tile_width as f32,
                            y: tiled_map.map.tile_height as f32,
                        };

                        let mesh_type = match tiled_map.map.orientation {
                            tiled::Orientation::Hexagonal => TilemapMeshType::Hexagon(HexType::Row),
                            tiled::Orientation::Isometric => {
                                TilemapMeshType::Isometric(IsoType::Diamond)
                            }
                            tiled::Orientation::Staggered => {
                                TilemapMeshType::Isometric(IsoType::Staggered)
                            }
                            tiled::Orientation::Orthogonal => TilemapMeshType::Square,
                        };

                        let mut tile_storage = TileStorage::empty(map_size);
                        let layer_entity = commands.spawn();
                        let layer_entity_id = layer_entity.id();

                        let tile_layer = match layer.layer_type() {
                            tiled::LayerType::TileLayer(v) => v,
                            tiled::LayerType::ObjectLayer(v) => {
                                for obj in v.object_data() {
                                    match obj.shape.clone() {
                                        tiled::ObjectShape::Rect { width, height } => {
                                            commands
                                                .spawn()
                                                .insert(Collider::cuboid(width / 2.0, height / 2.0))
                                                .insert_bundle(TransformBundle::from(
                                                    Transform::from_xyz(
                                                        obj.x - 800.0 + width / 2.0,
                                                        480.0 - obj.y - height / 2.0,
                                                        0.0,
                                                    ),
                                                ));
                                        }
                                        tiled::ObjectShape::Polygon { points } => {
                                            let mut ps = Vec::new();
                                            for (px, py) in points {
                                                ps.push(Vect::new(px + obj.x - 800.0, 480.0 - py - obj.y));
                                            }
                                            commands.spawn().insert(Collider::polyline(ps, None));
                                            // .insert_bundle(TransformBundle::from(Transform::from_xyz(obj.x - 800.0 + width / 2.0,  480.0 - obj.y - height / 2.0, 0.0)));
                                        }
                                        _ => {}
                                    }
                                }
                                continue;
                            }
                            _ => {
                                continue;
                            }
                        };

                        for x in 0..map_size.x {
                            for y in 0..map_size.y {
                                let mut mapped_y = x;
                                if tiled_map.map.orientation == tiled::Orientation::Orthogonal {
                                    mapped_y = (tiled_map.map.height - 1) as u32 - y;
                                }

                                // let mapped_x = x as usize;
                                // let mapped_y = mapped_y as usize;

                                let map_tile = match tile_layer.get_tile(x as i32, mapped_y as i32)
                                {
                                    Some(v) => v,
                                    None => continue,
                                };

                                // let tile_id = map_tile.gid - tileset.first_gid;

                                let tile_pos = TilePos { x, y };

                                let tile_entity = commands
                                    .spawn()
                                    .insert_bundle(TileBundle {
                                        position: tile_pos,
                                        tilemap_id: TilemapId(layer_entity_id),
                                        texture: TileTexture(map_tile.id()),
                                        flip: TileFlip {
                                            x: map_tile.flip_h,
                                            y: map_tile.flip_v,
                                            d: map_tile.flip_d,
                                        },
                                        ..Default::default()
                                    })
                                    .id();

                                tile_storage.set(&tile_pos, Some(tile_entity));
                            }
                        }

                        let tilemap_bundle = TilemapBundle {
                            grid_size,
                            size: map_size,
                            storage: tile_storage,
                            texture: TilemapTexture(
                                tiled_map.tilesets.get(&tileset.name).unwrap().clone_weak(),
                            ),
                            tile_size,
                            spacing: tile_spacing,
                            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                                &map_size,
                                &tile_size,
                                layer_index as f32,
                            ) * Transform::from_xyz(offset_x, -offset_y, 0.0),
                            mesh_type,
                            ..Default::default()
                        };

                        commands
                            .entity(layer_entity_id)
                            .insert_bundle(tilemap_bundle);

                        layer_storage
                            .storage
                            .insert(layer_index as u32, layer_entity_id);
                    }
                }
            }
        }
    }
}
