use serde;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameSubmodel {}

impl GameSubmodel {
    pub fn new() -> Self { Self{} }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CharacterSubmodel{
    move_frame_data: Vec<MoveFrameData>
}

impl CharacterSubmodel {
    pub fn new(move_frame_data: Vec<MoveFrameData>) -> Self {
        Self { move_frame_data }
    }
}
 

#[derive(strum::EnumString, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BlockType {
    Low,
    Mid,
    High
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MoveFrameData {
    move_name: String,
    hit_damage: f32,
    block_damage: f32,
    block_type: BlockType,
    flawless_block_damage: f32,
    startup_frames: u8,
    hit_advantage_frames: u8,
    active_frames: u8,
    block_advantage_frames: u8,
    recovery_frames: u8,
    flawless_block_advantage_frames: u8,
    cancel_frames: u8
}

impl MoveFrameData {
    pub fn new(
        move_name: String,
        hit_damage: f32,
        block_damage: f32,
        block_type: BlockType,
        flawless_block_damage: f32,
        startup_frames: u8,
        hit_advantage_frames: u8,
        active_frames: u8,
        block_advantage_frames: u8,
        recovery_frames: u8,
        flawless_block_advantage_frames: u8,
        cancel_frames: u8) -> Self
    {
        Self {
            move_name,
            hit_damage,
            block_damage,
            block_type,
            flawless_block_damage,
            startup_frames,
            hit_advantage_frames,
            active_frames,
            block_advantage_frames,
            recovery_frames,
            flawless_block_advantage_frames,
            cancel_frames
        }
    }
}