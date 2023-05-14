#![windows_subsystem = "windows"]

mod config;
use config::*;

mod game_rule;
use game_rule::*;



use bevy::asset::{HandleId};
use bevy::prelude::*;

use bevy::sprite::{MaterialMesh2dBundle};
use bevy::text::Text2dBounds;
use bevy::window::PresentMode;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Bevy 2048".to_string(),
				position: WindowPosition::Centered,
				width: WINDOW_WIDTH,
				height: WINDOW_HEIGHT,
				present_mode: PresentMode::AutoNoVsync,
				resizable: false,
				..default()
			},
			..default()
		}))
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_state(VictoryOrDefeat::NONE)
		// .add_startup_system(setup)
		.add_system_set(SystemSet::on_enter(VictoryOrDefeat::NONE).with_system(setup))
		.add_system_set(SystemSet::on_update(VictoryOrDefeat::NONE).with_system(keyboard_input))
		.add_system_set(SystemSet::on_enter(VictoryOrDefeat::DEFEAT).with_system(defeat_fn))
		.add_system_set(SystemSet::on_enter(VictoryOrDefeat::VICTORY).with_system(victory_function))
		// .add_system(keyboard_input)
		.run();
}

fn cell_color(cell_value: u32) -> bevy::render::color::Color {
	match cell_value {
		2 => COLOR_CELL_2.clone(),
		4 => COLOR_CELL_4.clone(),
		8 => COLOR_CELL_8.clone(),
		16 => COLOR_CELL_16.clone(),
		32 => COLOR_CELL_32.clone(),
		64 => COLOR_CELL_64.clone(),
		128 => COLOR_CELL_128.clone(),
		256 => COLOR_CELL_256.clone(),
		512 => COLOR_CELL_512.clone(),
		1024 => COLOR_CELL_1024.clone(),
		2048 => COLOR_CELL_2048.clone(),
		_ => COLOR_CELL_NULL.clone()
	}
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>
) {
	println!("setup");
	// 初始化存储数组
	let cell_value_save_temp: Vec<Vec<u32>> = init_cell_value_save();
	let mut cell_background_save: Vec<HandleId> = Vec::new();
	// 计算左上方格偏移
	let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
	let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	let y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;


	commands.spawn(Camera2dBundle::default());

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into()).into(),
		material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
		transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
		..default()
	});

	// 初始化文字信息
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: side_length / 2.0,
		color: COLOR_BROWN,
	};
	let box_size = Vec2::new(side_length, side_length);

	for i in 0..CELL_SIDE_NUM {
		for j in 0..CELL_SIDE_NUM {

			// 格中显示内容
			let mut text = "";
			if cell_value_save_temp[i as usize][j as usize] == 2 {
				text = "2";
			}

			let material_color = materials.add(ColorMaterial::from(cell_color(cell_value_save_temp[i as usize][j as usize])));
			cell_background_save.push(material_color.id());
			// 绑定格，根据数字来确定格的颜色
			commands.spawn(
				MaterialMesh2dBundle {
					mesh: meshes.add(shape::Box::new(side_length, side_length, 0.0).into()).into(),
					material: material_color,
					transform: Transform::from_xyz(
						x_offset + (j as f32) * (side_length + CELL_SPACE),
						y_offset - (i as f32) * (side_length + CELL_SPACE),
						0.0),
					..default()
				}
			);

			// 绑定数字
			commands.spawn((
				Text2dBundle {
					text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::CENTER),
					text_2d_bounds: Text2dBounds {
						// Wrap text in the rectangle
						size: box_size,
					},
					transform: Transform::from_xyz(
						x_offset + (j as f32) * (side_length + CELL_SPACE),
						y_offset - (i as f32) * (side_length + CELL_SPACE),
						1.0),
					..default()
				},
				CellValue
			));
		}
	}

	// 将存储数组设为资源
	commands.insert_resource(
		CellValueSave{
			value_save: cell_value_save_temp.clone(),
			cell_back_ground: cell_background_save,
			score: 0
		}
	);

	commands.spawn(
		Text2dBundle {
			text: Text::from_sections(
				[
					TextSection::new("SCORE\n", text_style.clone()),
					TextSection::new("0", text_style.clone()),
				]

			),
			text_2d_bounds: Text2dBounds {
				// Wrap text in the rectangle
				size: box_size,
			},
			transform: Transform::from_xyz(
				-WINDOW_WIDTH / 2.0,
				WINDOW_HEIGHT / 2.0,
				0.0,
			),
			..default()
		}
	);
}

fn keyboard_input(
	keyboard_input: Res<Input<KeyCode>>,
	asset_server: Res<AssetServer>,
	mut cell_value_save: ResMut<CellValueSave>,
	mut text_query: Query<&mut Text, With<CellValue>>,
	mut score_query: Query<&mut Text, Without<CellValue>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut app_state: ResMut<State<VictoryOrDefeat>>,
) {
	let mut moved = MoveDirection::NONE;
	if keyboard_input.just_pressed(KeyCode::Up) {
		moved = MoveDirection::UP;
	}
	if keyboard_input.just_pressed(KeyCode::Down) {
		moved = MoveDirection::DOWN;
	}
	if keyboard_input.just_pressed(KeyCode::Right) {
		moved = MoveDirection::RIGHT;
	}
	if keyboard_input.just_pressed(KeyCode::Left) {
		moved = MoveDirection::LEFT;
	}

	match moved {
		MoveDirection::NONE => return,
		_ => {
			println!("moved");
			let mut i = 0;
			move_value(moved, &mut cell_value_save);

			score_query.single_mut().sections[1].value = cell_value_save.score.to_string();

			let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
			let font = asset_server.load("fonts/FiraSans-Bold.ttf");
			let mut text_style = TextStyle {
				font,
				font_size: side_length / 2.0,
				color: COLOR_BROWN,
			};

			for mut text in text_query.iter_mut() {
				let cell_value_temp = cell_value_save.value_save[i / 4][i % 4];

				if cell_value_temp > 4 {
					text_style.color = COLOR_WHITE;
				} else {
					text_style.color = COLOR_BROWN;
				}

				if cell_value_temp != 0 {
					text.sections[0].style = text_style.clone();
					text.sections[0].value = cell_value_save.value_save[i / 4][i % 4].to_string();
				} else {
					text.sections[0].value = "".to_string();
				}
				materials.set_untracked(cell_value_save.cell_back_ground[i], ColorMaterial::from(cell_color(cell_value_save.value_save[i / 4][i % 4])));
				i += 1;
			}

			let result = check_result(&mut cell_value_save);
			match result {
				VictoryOrDefeat::VICTORY => {
					println!("victory");
					let _overwrite_set = app_state.overwrite_set(VictoryOrDefeat::VICTORY);
				},
				VictoryOrDefeat::DEFEAT => {
					println!("defeat");
					let _overwrite_set = app_state.overwrite_set(VictoryOrDefeat::DEFEAT);
				},
				VictoryOrDefeat::NONE => { println!("none"); Some(());}
			};
		}
	}
}

fn defeat_fn(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	cell_value_save: ResMut<CellValueSave>,
	mut _app_state: ResMut<State<VictoryOrDefeat>>,
	entities: Query<Entity, Without<Camera>>
) {
	for entity_query in &entities {
		commands.entity(entity_query).despawn();
	}
	let box_size = Vec2::new(WINDOW_HEIGHT, WINDOW_HEIGHT);
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: WINDOW_HEIGHT / 5.0,
		color: COLOR_BROWN,
	};

	let mut text = String::from("YOU  LOST\nSCORE: ");
	text.push_str(&cell_value_save.score.to_string());
	commands.spawn(Text2dBundle {
			text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::CENTER),
			text_2d_bounds: Text2dBounds {
				size: box_size,
			},
			..default()
		});
}

fn victory_function(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	cell_value_save: ResMut<CellValueSave>,
	_app_state: ResMut<State<VictoryOrDefeat>>,
	entities: Query<Entity, Without<Camera>>
) {
	for entity_query in &entities {
		commands.entity(entity_query).despawn();
	}
	let box_size = Vec2::new(WINDOW_HEIGHT, WINDOW_HEIGHT);
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: WINDOW_HEIGHT / 5.0,
		color: COLOR_BROWN,
	};

	let mut text = String::from("WINNER\nSCORE: ");
	text.push_str(&cell_value_save.score.to_string());
	commands.spawn(Text2dBundle {
			text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::CENTER),
			text_2d_bounds: Text2dBounds {
				size: box_size,
			},
			..default()
		});
}