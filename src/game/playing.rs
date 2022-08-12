use std::{collections::HashMap, time::Duration};

use super::{
    libs::{AnimationData, TiledMapBundle, TiledMapPlugin},
    loading::{AnimationAssets, TextureAssets},
    GameState,
};

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::{
    CoefficientCombineRule, Collider, Friction, GravityScale, LockedAxes, NoUserData, QueryFilter,
    RapierContext, RapierPhysicsPlugin, RigidBody, Velocity,
};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TiledMapPlugin);
        app.add_plugin(LookTransformPlugin);
        // app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());
        // app.add_plugin(bevy_rapier2d::prelude::RapierDebugRenderPlugin::default());
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
        app.add_event::<MoveCameraEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Start).with_system(spawn_scene))
            .add_system_set(
                SystemSet::on_update(GameState::Start)
                    .with_system(animate_sprite)
                    .with_system(move_camera),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Status {
    Idle,
    Walk,
    Run,
    Jump1,
    Jump2,
    Jump3,
    Jump4,
    Jump5,
    // Attack,
}

impl Status {
    fn to_str(&self) -> &str {
        match self {
            Status::Idle => "idle",
            Status::Walk => "walk",
            Status::Run => "run",
            Status::Jump1 => "jump_1",
            Status::Jump2 => "jump_2",
            Status::Jump3 => "jump_3",
            Status::Jump4 => "jump_4",
            Status::Jump5 => "jump_5",
            // Status::Attack => "attack",
        }
    }
}

#[derive(Clone)]
struct Args {
    speed: f32,
    velocity_y: f32,
    is_ground: bool,
    jump: bool,
    last_frame: bool,
}

#[derive(Component)]
struct StateMachine {
    state: Status,
    args: Args,
    index: i32,
}

impl StateMachine {
    fn run(&mut self) -> bool {
        let a = match self.state {
            Status::Idle => {
                if self.args.speed.abs() > 0.1 && self.args.speed.abs() < 0.5 {
                    Status::Walk
                } else if self.args.speed.abs() > 0.5 {
                    Status::Run
                } else if self.args.jump {
                    self.args.jump = false;
                    Status::Jump1
                } else if !self.args.is_ground {
                    Status::Jump4
                } else {
                    self.state
                }
            }
            Status::Walk => {
                if self.args.speed.abs() < 0.1 {
                    Status::Idle
                } else if self.args.speed.abs() > 0.5 {
                    Status::Run
                } else if self.args.jump {
                    self.args.jump = false;
                    Status::Jump1
                } else if !self.args.is_ground {
                    Status::Jump4
                } else {
                    self.state
                }
            }
            Status::Run => {
                if self.args.speed.abs() < 0.1 {
                    Status::Idle
                } else if self.args.speed.abs() < 0.5 {
                    Status::Walk
                } else if self.args.jump {
                    self.args.jump = false;
                    Status::Jump1
                } else if !self.args.is_ground {
                    Status::Jump4
                } else {
                    self.state
                }
            }
            Status::Jump1 => {
                if self.args.last_frame {
                    Status::Jump2
                } else {
                    self.state
                }
            }
            Status::Jump2 => {
                if self.args.velocity_y < 0.0 {
                    Status::Jump3
                } else {
                    self.state
                }
            }
            Status::Jump3 => {
                if self.args.last_frame {
                    Status::Jump4
                } else {
                    self.state
                }
            }
            Status::Jump4 => {
                if self.args.is_ground {
                    Status::Jump5
                } else {
                    self.state
                }
            }
            Status::Jump5 => {
                if self.args.last_frame {
                    Status::Idle
                } else {
                    self.state
                }
            } // _ => self.state,
        };

        self.args.last_frame = false;

        if self.state != a {
            self.state = a;
            self.index = 0;
            return true;
        }
        false
    }
}

fn spawn_scene(
    mut commands: Commands,
    ass: Res<AnimationAssets>,
    anim: Res<Assets<AnimationData>>,
    asset_server: Res<AssetServer>,
    texture_assets: Res<TextureAssets>,
) {
    let pos = Vec3 {
        x: -50.0,
        y: 0.0,
        z: 0.0,
    };

    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::Auto {
                    min_width: 800.0,
                    min_height: 480.0,
                },
                ..default()
            },
            ..default()
        })
        .insert_bundle(LookTransformBundle {
            transform: LookTransform::new(pos, pos),
            smoother: Smoother::new(0.9),
        });

    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(3.0, 3.0, 1.0),
            translation: Vec3::new(0.0, -100.0, -1.0),
            ..default()
        },
        texture: texture_assets.bg.clone(),
        ..default()
    });

    commands.spawn_bundle(TiledMapBundle {
        tiled_map: asset_server.load("tiled/01.tmx"),
        ..default()
    });

    let n = anim.get(&ass.player01).unwrap();

    // commands
    //     .spawn()
    //     .insert(Collider::cuboid(640.0, 10.0))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -230.0, 0.0)));

    // commands
    //     .spawn()
    //     .insert(Collider::cuboid(10.0, 230.0))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(
    //         -460.0, -80.0, 0.0,
    //     )));

    // commands
    //     .spawn()
    //     .insert(Collider::cuboid(10.0, 230.0))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(
    //         460.0, -80.0, 0.0,
    //     )));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            texture_atlas: n.atlas.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1.0, true)))
        .insert(StateMachine {
            state: Status::Idle,
            args: Args {
                speed: 0.0,
                velocity_y: 0.0,
                is_ground: true,
                jump: false,
                last_frame: false,
            },
            index: 0,
        })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(7.0))
        .insert(Collider::capsule_y(17.0, 15.0))
        .insert(Velocity::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        });

    commands.spawn_bundle(TextBundle {
        text: Text::from_section(
            r#"
                左右控制移动
                按住 Z + 方向键 奔跑
                空格键 跳跃
                按住 空格键 跳的更高
            "#,
            TextStyle {
                font_size: 24.0,
                font: asset_server.load("fonts/MSYH.TTF"),
                ..default()
            },
        ),
        style: Style { ..default() },
        ..default()
    });
}

struct MoveCameraEvent {
    target: Vec2,
}

fn move_camera(mut events: EventReader<MoveCameraEvent>, mut query: Query<(&mut LookTransform,)>) {
    for eve in events.iter() {
        for (mut look_transform,) in &mut query {
            look_transform.target = Vec3::new(eve.target.x, eve.target.y, 0.0);
            look_transform.eye = Vec3::new(eve.target.x, eve.target.y, 3.0);
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    animation: Res<AnimationAssets>,
    animation_assets: Res<Assets<AnimationData>>,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_context: Res<RapierContext>,
    mut move_events: EventWriter<MoveCameraEvent>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut StateMachine,
        &mut Velocity,
        &mut GravityScale,
        &Transform,
    )>,
) {
    let anim = animation_assets.get(&animation.player01).unwrap();
    let mut animation_map = HashMap::new();

    for armature in anim.ske.armature.iter() {
        let skin = armature.skin.first().unwrap();
        for animation_fragment in armature.animation.iter() {
            animation_map.insert(
                animation_fragment.name.clone(),
                (
                    animation_fragment,
                    skin.slot.first().unwrap().display.clone(),
                    armature.frame_rate,
                ),
            );
        }
    }

    for (mut timer, mut sprite, mut player, mut velocity, mut gravity, transform) in &mut query {
        timer.tick(time.delta());

        let mut is_timer_done = timer.just_finished();

        if let Some(_) = rapier_context.cast_ray(
            transform.translation.truncate(),
            Vec2 { x: 0.0, y: -1.0 },
            32.0 + 5.0,
            false,
            QueryFilter::default().exclude_dynamic(),
        ) {
            if !player.args.is_ground {
                player.args.is_ground = true;
                gravity.0 = 7.0;
            }
        } else {
            player.args.is_ground = false;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            sprite.flip_x = true;
            player.args.speed = if keyboard_input.pressed(KeyCode::Z) {
                -0.6
            } else {
                -0.4
            };
        } else if keyboard_input.pressed(KeyCode::Right) {
            sprite.flip_x = false;
            player.args.speed = if keyboard_input.pressed(KeyCode::Z) {
                0.6
            } else {
                0.4
            };
        } else {
            player.args.speed = 0.0;
        };

        player.args.velocity_y = velocity.linvel.y;

        if keyboard_input.just_pressed(KeyCode::Space) && player.args.is_ground {
            velocity.linvel.y = 600.0;
            player.args.jump = true;
        }

        if keyboard_input.just_released(KeyCode::Space) && !player.args.is_ground {
            gravity.0 = 15.0;
        }

        // player.args.jump -= player.args.jump * time.delta_seconds() * 30.0;

        if player.run() {
            let (_, _, frame_rate) = animation_map.get(player.state.to_str()).unwrap().clone();
            timer.set_duration(Duration::from_secs_f32(1.0 / frame_rate));
            // info!("切换 {:?}", player.state);
            is_timer_done = true;
        }

        let (state, slot, frame_rate) = animation_map.get(player.state.to_str()).unwrap().clone();

        let display_frame = state.slot.first().unwrap().display_frame.clone();

        if is_timer_done {
            player.index += 1;
            if player.index >= display_frame.len() as i32 {
                player.index = 0;
                player.args.last_frame = true;
            }
        }

        let display = display_frame.get(player.index as usize).unwrap();

        let frame = slot.get(display.value as usize).unwrap();
        let mut move_v = 0.0;

        if let Some(tr) = frame.transform.clone() {
            move_v = tr.x;
            if display.value > 0 {
                let p_frame = slot.get(display.value as usize - 1).unwrap();
                if let Some(p_tr) = p_frame.transform.clone() {
                    move_v = tr.x - p_tr.x;
                }
            }

            if sprite.flip_x {
                move_v *= -1.0;
            }

            // transform.translation.x += move_v * state.duration as f32 * time.delta_seconds();
        }

        if !player.args.is_ground {
            velocity.linvel.x += (player.args.speed * 200.0 - velocity.linvel.x) * 0.7;
        } else {
            velocity.linvel.x += (move_v * frame_rate - velocity.linvel.x) * 0.7;
        }

        if is_timer_done {
            let sp_index = anim.map.get(&frame.name).unwrap().clone();
            sprite.index = sp_index as usize;
            // transform.translation.x += move_v;
            // info!(player.index, move_v, sp_index, frame.name);
        }

        let translation = transform.translation;

        move_events.send(MoveCameraEvent {
            target: Vec2 {
                x: translation.x.clamp(-400.0, 400.0),
                y: translation.y.clamp(-240.0, 240.0),
            },
        });
    }
}

// app.add_plugin(LookTransformPlugin);
// app.add_plugin(UnrealCameraPlugin::default());

// .add_startup_system(setup);
// .add_plugin(TilemapPlugin)
// .add_plugin(libs::TiledMapPlugin)

// app.insert_resource(AmbientLight {
//     color: Color::WHITE,
//     brightness: 1.0 / 5.0f32,
// });
// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let eye = Vec3::new(0.7, 0.7, 1.2);
//     let target = Vec3::new(0.0, 0.8, 0.0);
//     commands
//         .spawn_bundle(LookTransformBundle {
//             transform: LookTransform::new(eye, target),
//             smoother: Smoother::new(0.9),
//         })
//         .insert_bundle(Camera3dBundle { ..default() })
//         .insert_bundle(UnrealCameraBundle::new(
//             UnrealCameraController::default(),
//             eye,
//             target,
//         ));
//     const HALF_SIZE: f32 = 1.0;
//     commands.spawn_bundle(DirectionalLightBundle {
//         directional_light: DirectionalLight {
//             shadow_projection: OrthographicProjection {
//                 left: -HALF_SIZE,
//                 right: HALF_SIZE,
//                 bottom: -HALF_SIZE,
//                 top: HALF_SIZE,
//                 near: -10.0 * HALF_SIZE,
//                 far: 10.0 * HALF_SIZE,
//                 ..default()
//             },
//             shadows_enabled: true,
//             ..default()
//         },
//         ..default()
//     });
//     commands.spawn_bundle(SceneBundle {
//         scene: asset_server.load("models/linlin01.glb#Scene0"),
//         ..default()
//     });
// }

// fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn_bundle(Camera2dBundle::default());

//     let map_handle: Handle<libs::TiledMap> = asset_server.load("tiled/01.tmx");

//     commands.spawn().insert_bundle(libs::TiledMapBundle {
//         tiled_map: map_handle,
//         ..Default::default()
//     });
// }

// fn update() {

// }

// fn animate_light_direction(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<DirectionalLight>>,
// ) {
//     for mut transform in &mut query {
//         transform.rotation = Quat::from_euler(
//             EulerRot::ZYX,
//             0.0,
//             time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
//             -std::f32::consts::FRAC_PI_4,
//         );
//     }
// }
