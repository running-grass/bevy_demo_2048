#![windows_subsystem = "windows"]

mod config;
use config::*;

mod game_rule;
use game_rule::*;

use bevy::asset::HandleId;
use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::text::Text2dBounds;
use bevy::window::{PresentMode, WindowResolution};

// use bevy::render::settings::WgpuSettings;
fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2048".to_string(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::AutoNoVsync,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_state::<VictoryOrDefeat>()
        .add_event::<MoveEvent>()
        .add_event::<DateChangeEvent>()
        .add_system(setup.in_schedule(OnEnter(VictoryOrDefeat::NONE)))
        .add_system(defeat_fn.in_schedule(OnEnter(VictoryOrDefeat::DEFEAT)))
        .add_system(victory_function.in_schedule(OnEnter(VictoryOrDefeat::VICTORY)))
        .add_systems(
            (
                keyboard_input_system,
                move_handler_system,
                sync_data_to_display_system,
            )
                .in_set(OnUpdate(VictoryOrDefeat::NONE)),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 初始化存储数组
    let cell_value_save_temp: Vec<Vec<u32>> = init_cell_value_save();
    let mut cell_background_save: Vec<HandleId> = Vec::new();
    // 计算左上方格偏移
    let side_length: f32 =
        (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;

    let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
    let y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
    x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;

    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into())
            .into(),
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

            let material_color = materials.add(ColorMaterial::from(cell_color(
                cell_value_save_temp[i as usize][j as usize],
            )));
            cell_background_save.push(material_color.id());
            // 绑定格，根据数字来确定格的颜色
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Box::new(side_length, side_length, 0.0).into())
                    .into(),
                material: material_color,
                transform: Transform::from_xyz(
                    x_offset + (j as f32) * (side_length + CELL_SPACE),
                    y_offset - (i as f32) * (side_length + CELL_SPACE),
                    0.0,
                ),
                ..default()
            });

            // 绑定数字
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(text, text_style.clone())
                        .with_alignment(TextAlignment::Center),
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: box_size,
                    },
                    transform: Transform::from_xyz(
                        x_offset + (j as f32) * (side_length + CELL_SPACE),
                        y_offset - (i as f32) * (side_length + CELL_SPACE),
                        1.0,
                    ),
                    ..default()
                },
                CellValue,
            ));
        }
    }

    // 将存储数组设为资源
    commands.insert_resource(CellValueSave {
        value_save: cell_value_save_temp.clone(),
        cell_back_ground: cell_background_save,
        score: 0,
    });

    commands.spawn(Text2dBundle {
        text: Text::from_sections([
            TextSection::new("SCORE\n", text_style.clone()),
            TextSection::new("0", text_style.clone()),
        ]).with_alignment(TextAlignment::Right),
        text_2d_bounds: Text2dBounds {
            // Wrap text in the rectangle
            size: box_size,
        },
        transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0 + side_length / 1.5, WINDOW_HEIGHT / 2.0 - side_length / 2.0, 0.0),
        // global_transform: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
