use std::{collections::HashMap, fmt::Debug};

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    sprite::Rect,
    utils::BoxedFuture,
};

use super::dragon_models::{SkeRoot, TexRoot};

#[derive(Default)]
pub struct AnimationLoader;

impl AssetLoader for AnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let ske_root: SkeRoot = serde_json::from_slice(bytes)?;
            let obj_path = load_context
                .path()
                .to_str()
                .unwrap_or_default()
                .strip_suffix(".anim_ske.json")
                .unwrap_or_default();
            let tex_root: TexRoot = serde_json::from_slice(
                load_context
                    .read_asset_bytes(format!("{}_tex.json", obj_path))
                    .await?
                    .as_slice(),
            )?;

            let animation_data = AnimationData {
                ske: ske_root,
                tex: tex_root,
                path_prefix: obj_path.to_string(),
                atlas: Handle::default(),
                map: HashMap::new(),
            };
            // info!("load data done: {:?}", animation_data);
            load_context.set_default_asset(LoadedAsset::new(animation_data));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["anim_ske.json"]
    }
}

#[derive(Default, Debug, Clone, TypeUuid, Component)]
#[uuid = "eb89c226-9f88-4f1c-8b16-08981b602b4b"]
pub struct AnimationData {
    pub ske: SkeRoot,
    tex: TexRoot,
    path_prefix: String,
    pub atlas: Handle<TextureAtlas>,
    pub map: HashMap<String, u32>,
}

pub struct Animation {}

impl FromWorld for Animation {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();

        let mut datas = cell.get_resource_mut::<Assets<AnimationData>>().unwrap();

        let server = cell.get_resource::<AssetServer>().unwrap();

        let mut atlas_assets = cell.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

        for (_, anim) in datas.iter_mut() {
            let texture: Handle<Image> = server.load(&format!("{}_tex.png", anim.path_prefix));

            let mut atlas = TextureAtlas::new_empty(
                texture,
                Vec2 {
                    x: anim.tex.width as f32,
                    y: anim.tex.height as f32,
                },
            );

            for it in anim.tex.sub_texture.iter() {
                let r = Rect {
                    min: Vec2 { x: it.x, y: it.y },
                    max: Vec2 {
                        x: it.x + it.width,
                        y: it.y + it.height,
                    },
                };
                let i = atlas.add_texture(r);
                anim.map.insert(it.name.clone(), i as u32);
            }

            let atlas_handle = atlas_assets.add(atlas);

            anim.atlas = atlas_handle;
        }

        Self {}
    }
}

// mod test_load_mod {

//     #[test]
//     fn test_load() {
//         use crate::game::libs::dragon_models::{SkeRoot, TexRoot};
//         use std::path;

//         let data = std::fs::read_to_string("assets/animation/player01.anim_ske.json").unwrap();
//         let result: SkeRoot = serde_json::from_str(&data).unwrap();

//         println!("result = {:?}", result);

//         let data = std::fs::read_to_string("assets/animation/player01_tex.json").unwrap();
//         let result: TexRoot = serde_json::from_str(&data).unwrap();

//         println!("result = {:?}", result);

//         let mut p = path::PathBuf::new();

//         p.push("assets/animation/player01.anim_ske.json");

//         if let Some(v) = p.to_str() {
//             let b = v.strip_suffix("anim_ske.json");
//             println!("r = {:?}", b);
//         }

//         println!("{:?}", p.extension());
//     }
// }

// pub struct Condition<Args> {
//     fun: Box<dyn FnMut(Args, PlayState) -> bool>,
// }

// pub trait ConditionFunc<Args> {
//     fn get_func(self) -> Condition<Args>;
// }

// impl<F, Args> ConditionFunc<Args> for F
// where
//     F: FnMut(Args, PlayState) -> bool + 'static,
// {
//     fn get_func(self) -> Condition<Args> {
//         Condition {
//             fun: Box::new(self),
//         }
//     }
// }

// pub struct PlayState {
//     done: bool,
// }

// #[derive()]
// pub struct AnimationStatus<S, A>
// where
//     S: Eq + Hash,
// {
//     cur: S,
//     map: HashMap<S, Vec<(S, Condition<A>)>>,
// }

// impl<S, A> AnimationStatus<S, A>
// where
//     S: Eq + Hash + Clone + Copy + Debug,
//     A: Clone,
// {
//     pub fn new(start: S) -> Self {
//         Self {
//             cur: start,
//             map: HashMap::new(),
//         }
//     }

//     pub fn add_edge(&mut self, from: S, to: S, condition: impl ConditionFunc<A>) {
//         self.map
//             .entry(from)
//             .or_insert(Vec::new())
//             .push((to, condition.get_func()));
//     }

//     pub fn run(&mut self, arg: A) {
//         for (to, cond) in self.map.get_mut(&self.cur).unwrap() {
//             if cond.fun.call_mut((arg.clone(), PlayState { done: true })) {
//                 self.cur = to.clone();
//             }
//         }
//         println!("cur => {:?}", self.cur);
//     }
// }

// pub fn on_done<T>(_: T, s: PlayState) -> bool {
//     s.done
// }

// mod t {

//     use super::{on_done, AnimationStatus};

//     #[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
//     enum Status {
//         Idle,
//         Walk,
//         Run,
//         Jump,
//         Attack,
//     }

//     #[derive(Clone)]
//     struct Args {
//         speed: f32,
//     }

//     fn t() {
//         let mut a = AnimationStatus::new(Status::Idle);

//         a.add_edge(Status::Idle, Status::Walk, |x: Args, _| x.speed > 0.1);
//         a.add_edge(Status::Walk, Status::Idle, |x: Args, _| x.speed < 0.1);
//         a.add_edge(Status::Walk, Status::Run, |x: Args, _| x.speed > 3.0);
//         a.add_edge(Status::Run, Status::Jump, on_done);
//         a.add_edge(Status::Jump, Status::Idle, on_done);

//         a.run(Args { speed: 1.0 });
//         a.run(Args { speed: 0.0 });
//     }

//     fn modulo(x: i32, n: i32) -> i32 {
//         (x % n + n) % n
//     }

//     #[test]
//     fn tt() {
//         println!("{}", modulo(-2, 10));
//         t();
//     }
// }
