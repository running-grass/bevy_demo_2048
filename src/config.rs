use bevy::asset::HandleId;
use bevy::prelude::*;

pub static WINDOW_WIDTH: f32 = 800.0;
pub static WINDOW_HEIGHT: f32 = 600.0;
pub static CELL_SPACE: f32 = 6.0;
pub static CELL_SIDE_NUM: u32 = 4;
pub static COLOR_BACKGROUND: Color = Color::rgb(187.0 / 255.0, 173.0 / 255.0, 160.0 / 255.0);
pub static COLOR_CELL_NULL: Color = Color::rgb(204.0 / 255.0, 192.0 / 255.0, 178.0 / 255.0);
pub static COLOR_CELL_2: Color = Color::rgb(238.0 / 255.0, 228.0 / 255.0, 218.0 / 255.0);
pub static COLOR_CELL_4: Color = Color::rgb(236.0 / 255.0, 224.0 / 255.0, 204.0 / 255.0);
pub static COLOR_CELL_8: Color = Color::rgb(242.0 / 255.0, 176.0 / 255.0, 120.0 / 255.0);
pub static COLOR_CELL_16: Color = Color::rgb(245.0 / 255.0, 148.0 / 255.0, 98.0 / 255.0);
pub static COLOR_CELL_32: Color = Color::rgb(245.0 / 255.0, 124.0 / 255.0, 95.0 / 255.0);
pub static COLOR_CELL_64: Color = Color::rgb(245.0 / 255.0, 92.0 / 255.0, 60.0 / 255.0);
pub static COLOR_CELL_128: Color = Color::rgb(235.0 / 255.0, 208.0 / 255.0, 113.0 / 255.0);
pub static COLOR_CELL_256: Color = Color::rgb(239.0 / 255.0, 203.0 / 255.0, 97.0 / 255.0);
pub static COLOR_CELL_512: Color = Color::rgb(239.0 / 255.0, 193.0 / 255.0, 45.0 / 255.0);
pub static COLOR_CELL_1024: Color = Color::rgb(239.0 / 255.0, 197.0 / 255.0, 63.0 / 255.0);
pub static COLOR_CELL_2048: Color = Color::rgb(238.0 / 255.0, 194.0 / 255.0, 46.0 / 255.0);

pub static COLOR_BROWN: Color = Color::rgb(120.0 / 255.0, 110.0 / 255.0, 100.0 / 255.0);
pub static COLOR_WHITE: Color = Color::rgb(245.0 / 255.0, 250.0 / 255.0, 240.0 / 255.0);

pub enum MoveDirection {
	None,
	Up,
	Down,
	Left,
	Right
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum VictoryOrDefeat {
	Victory,
	#[default]
	None,
	Defeat
}


#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugState {
	Yes,
	#[default]
	No,
}


#[derive(Resource)]
pub struct CellValueSave {
	pub(crate) value_save: Vec<Vec<u32>>,
	pub(crate) cell_back_ground: Vec<HandleId>,
	pub(crate) score: u32
}


#[derive(Component)]
pub struct CellValue;

pub struct MoveEvent(pub MoveDirection);

pub struct DateChangeEvent;



pub fn cell_color(cell_value: u32) -> bevy::render::color::Color {
	match cell_value {
			2 => COLOR_CELL_2,
			4 => COLOR_CELL_4,
			8 => COLOR_CELL_8,
			16 => COLOR_CELL_16,
			32 => COLOR_CELL_32,
			64 => COLOR_CELL_64,
			128 => COLOR_CELL_128,
			256 => COLOR_CELL_256,
			512 => COLOR_CELL_512,
			1024 => COLOR_CELL_1024,
			2048 => COLOR_CELL_2048,
			_ => COLOR_CELL_NULL,
	}
}