use serde::Deserialize;
use serde::Serialize;

use bevy::reflect::TypeUuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypeUuid)]
#[serde(rename_all = "camelCase")]
#[uuid = "a29d8036-1440-468f-aeb9-049e140cc512"]
pub struct SkeRoot {
    pub frame_rate: i64,
    pub name: String,
    pub version: String,
    pub compatible_version: String,
    pub armature: Vec<Armature>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Armature {
    #[serde(rename = "type")]
    pub type_field: String,
    pub frame_rate: f32,
    pub name: String,
    pub aabb: Aabb,
    pub bone: Vec<Bone>,
    pub slot: Vec<ArmatureSlot>,
    pub skin: Vec<Skin>,
    pub animation: Vec<Animation>,
    pub default_actions: Vec<DefaultAction>,
    pub canvas: Option<Canvas>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aabb {
    pub x: f64,
    pub y: f64,
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bone {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmatureSlot {
    pub name: String,
    pub parent: String,
    #[serde(default)]
    pub display_index: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub slot: Vec<SkinSlot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinSlot {
    pub name: String,
    pub display: Vec<Display>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    pub name: String,
    pub transform: Option<Transform>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    pub duration: f32,
    #[serde(default)]
    pub play_times: i32,
    pub name: String,
    pub slot: Vec<AnimationSlot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimationSlot {
    pub name: String,
    pub display_frame: Vec<DisplayFrame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayFrame {
    #[serde(default)]
    pub value: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultAction {
    pub goto_and_play: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypeUuid)]
#[serde(rename_all = "camelCase")]
#[uuid = "5259c3bc-a104-4bd2-bb60-b79b930f06ec"]
pub struct TexRoot {
    pub width: u32,
    #[serde(rename = "SubTexture")]
    pub sub_texture: Vec<SubTexture>,
    pub height: u32,
    pub name: String,
    pub image_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubTexture {
    pub frame_x: i32,
    pub frame_height: u32,
    pub y: f32,
    pub frame_y: i32,
    pub frame_width: u32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub x: f32,
}
