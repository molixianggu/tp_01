use std::collections::HashMap;
use evalexpr::Function;
use serde::Deserialize;
use serde::Serialize;

use bevy::reflect::TypeUuid;
use evalexpr::{ContextWithMutableFunctions, ContextWithMutableVariables};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    IntArg {
        name: String,
        value: i64,
    },
    FloatArg {
        name: String,
        value: f64,
    },
    BooleanArg {
        name: String,
        value: bool,
    },
    EnumArg {
        name: String,
        value: String,
        optional: Vec<String>,
    },
    TriggerArg {
        name: String,
        value: bool,
    },
}

#[derive(Debug, Clone)]
pub struct StateDetails {
    pub sub_name: String,
    pub play_name: String,
    pub index: u32,
}

impl StateDetails {
    pub fn new(sub_name: String, play_name: String, index: u32) -> Self {
        Self {
            sub_name,
            play_name,
            index,
        }
    }

    fn copy(&self) -> Self {
        Self {
            sub_name: self.sub_name.clone(),
            play_name: self.play_name.clone(),
            index: self.index,
        }
    }
}

pub type ConditionList = Vec<(String, evalexpr::Node)>;

pub type StatusSet = HashMap<String, ConditionList>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Substatus {
    default: String,
    content: StatusSet,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AnimationStateMachineData {
    name: String,
    args: Vec<FieldType>,
    default: String,
    substatus: HashMap<String, Substatus>,
}

#[derive(Debug)]
pub struct AnimationStateMachine {
    current_state: Vec<StateDetails>,
    data: std::rc::Rc<AnimationStateMachineData>,
    ctx: evalexpr::HashMapContext,
    last_frame: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl AnimationStateMachine {
    pub fn new(data: std::rc::Rc<AnimationStateMachineData>) -> Self {
        let mut s = Self {
            current_state: vec![],
            data,
            ctx: evalexpr::HashMapContext::new(),
            last_frame: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        };

        if let Some(cur) = s.data.substatus.get(&s.data.default) {
            s.current_state.push(StateDetails::new(
                s.data.default.clone(),
                cur.default.clone(),
                0,
            ));
        }

        for field in s.data.args.clone() {
            let (identifier, v) = match field {
                FieldType::IntArg { name, value } => (name, evalexpr::Value::Int(value)),
                FieldType::FloatArg { name, value } => (name, evalexpr::Value::Float(value)),
                FieldType::BooleanArg { name, value } => (name, evalexpr::Value::Boolean(value)),
                FieldType::EnumArg {
                    name,
                    value,
                    optional: _,
                } => (name, evalexpr::Value::String(value)),
                FieldType::TriggerArg { name, value } => (name, evalexpr::Value::Boolean(value)),
            };
            let _ = s.ctx.set_value(identifier, v);
        }

        {
            let last_frame = s.last_frame.clone();
            let _ = s.ctx.set_function(
                "last_frame".to_string(),
                Function::new(move |_v| -> evalexpr::EvalexprResult<evalexpr::Value> {
                    evalexpr::EvalexprResult::Ok(evalexpr::Value::Boolean(
                        last_frame.load(std::sync::atomic::Ordering::SeqCst),
                    ))
                }),
            );
        }
        s
    }

    pub fn run(&mut self) -> Result<Option<(StateDetails, StateDetails)>, ()> {
        let mut running_state: ConditionList = vec![];

        let state = match self.current_state.last() {
            Some(v) => v.copy(),
            None => {
                return Err(());
            }
        };

        if let Some(sub) = self.data.substatus.get(&state.sub_name) {
            running_state.clear();
            if let Some(content) = sub.content.get("") {
                running_state.extend(content.clone());
            }
            if let Some(content) = sub.content.get(&state.play_name) {
                running_state.extend(content.clone());
            }
        }

        let mut is_change = false;

        for (to, cond) in running_state {
            if let Some(state) = self.current_state.last() {
                if state.play_name == to {
                    continue;
                }
            }

            if let evalexpr::EvalexprResult::Ok(evalexpr::Value::Boolean(b)) =
                cond.eval_with_context(&self.ctx)
            {
                if !b {
                    continue;
                }

                lazy_static::lazy_static! {
                    static ref COMMAND_REG: regex::Regex = regex::Regex::new(r"([.><-]?)(\w*)\.?(\w*)\.?(\d*)").unwrap();
                }

                let (flag, sub_name, play_name, set_index) =
                    if let Some(cap) = COMMAND_REG.captures_iter(&to).next() {
                        (
                            cap[1].to_string(),
                            cap[2].to_string(),
                            cap[3].to_string(),
                            cap[4].parse().unwrap_or(0u32),
                        )
                    } else {
                        ("".to_string(), "".to_string(), "".to_string(), 0u32)
                    };

                if flag == "<" {
                    self.current_state.pop();
                    if let Some(state) = self.current_state.last_mut() {
                        if let Some(sub) = self.data.substatus.get(&state.sub_name) {
                            state.play_name = sub.default.clone();
                            state.index = set_index;
                        }
                    }
                } else if flag == ">" {
                    if let Some(sub) = self.data.substatus.get(&sub_name) {
                        if !play_name.is_empty() {
                            self.current_state
                                .push(StateDetails::new(sub_name, play_name, set_index));
                        } else {
                            self.current_state.push(StateDetails::new(
                                sub_name,
                                sub.default.clone(),
                                set_index,
                            ));
                        }
                    }
                } else if flag == "-" {
                    if let Some(sub) = self.data.substatus.get(&sub_name) {
                        if let Some(state) = self.current_state.last_mut() {
                            if !play_name.is_empty() {
                                *state = StateDetails::new(sub_name, play_name, set_index);
                            } else {
                                *state =
                                    StateDetails::new(sub_name, sub.default.clone(), set_index);
                            }
                        }
                    }
                } else if flag == "." {
                    if let Some(state) = self.current_state.last_mut() {
                        state.index = 0;
                    }
                } else if flag.is_empty() {
                    if let Some(state) = self.current_state.last_mut() {
                        if !sub_name.is_empty() && play_name.is_empty() {
                            state.play_name = sub_name;
                            state.index = set_index;
                        } else if !sub_name.is_empty() && !play_name.is_empty() {
                            state.sub_name = sub_name;
                            state.play_name = play_name;
                            state.index = set_index;
                        }
                    }
                }

                is_change = true;

                break;
            }
        }


        self.last_frame
            .store(false, std::sync::atomic::Ordering::SeqCst);

        if is_change {
            for arg in self.data.args.clone() {
                if let FieldType::TriggerArg { name, value: _ } = arg {
                    self.set_bool(name.clone(), false);
                }
            }
            if let Some(change_state) = self.current_state.last() {
                println!("切换状态: {:?} => {:?}", state, change_state);
                return Ok(Some((state, change_state.clone())));
            }
        }
        Ok(None)
    }

    pub fn set_last_frame(&self) {
        self.last_frame
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn state(&self) -> Option<&StateDetails> {
        self.current_state.last()
    }

    pub fn set_flaot(&mut self, identifier: String, value: f64) {
        let _ = self
            .ctx
            .set_value(identifier, evalexpr::Value::Float(value));
    }

    pub fn set_int(&mut self, identifier: String, value: i64) {
        let _ = self.ctx.set_value(identifier, evalexpr::Value::Int(value));
    }

    pub fn set_bool(&mut self, identifier: String, value: bool) {
        let _ = self
            .ctx
            .set_value(identifier, evalexpr::Value::Boolean(value));
    }

    pub fn set_trigger(&mut self, identifier: String) {
        let _ = self
            .ctx
            .set_value(identifier, evalexpr::Value::Boolean(true));
    }
}

#[test]
fn test_load_sm() {
    use crate::game::libs::dragon_models::{AnimationStateMachine, AnimationStateMachineData};

    let data = std::fs::read_to_string("assets/animation/player01_sm.ron").unwrap();
    let result: AnimationStateMachineData = ron::de::from_str(&data).unwrap();
    let result = std::rc::Rc::new(result);
    // println!("res = {:?}", result);

    let mut asm = AnimationStateMachine::new(result);

    asm.run();
    asm.run();

    asm.set_bool("is_ground".to_string(), true);

    asm.run();
    asm.set_last_frame();
    asm.run();
    asm.run();
    asm.run();

    asm.set_flaot("input_x".to_string(), 0.5);
    asm.run();
    asm.run();
    asm.set_last_frame();
    asm.run();
    asm.set_last_frame();
    asm.run();

    asm.set_trigger("jump".to_string());
    asm.set_bool("is_ground".to_string(), true);

    asm.run();

    println!("v = {:?}", asm.current_state);

    asm.set_last_frame();

    asm.run();

    asm.run();

    asm.set_flaot("velocity_y".to_string(), -1.5);

    println!("v = {:?}", asm.current_state);

    asm.run();

    asm.run();

    println!("v = {:?}", asm.current_state);

    asm.run();

    asm.set_last_frame();

    asm.run();

    asm.set_last_frame();

    asm.run();

    asm.run();

    println!("v = {:?}", asm.current_state);

    asm.set_last_frame();
    asm.run();

    println!("v = {:?}", asm.current_state);

    asm.set_bool("is_ground".to_string(), false);

    asm.run();

    println!("v = {:?}", asm.current_state);

    // let v = result.fsm.get("idle").unwrap().get(0).unwrap().1.clone();

    // println!("v = {v:?}");

    // let mut ctx = evalexpr::HashMapContext::new();

    // ctx.set_value("input_x".to_string(), evalexpr::Value::Float(0.4));

    // let r = v.eval_with_context(&ctx).unwrap();

    // println!("{r:?}");

    // evalexpr::build_operator_tree()
}
