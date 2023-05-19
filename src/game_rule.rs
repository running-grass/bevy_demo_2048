use bevy::{prelude::*, text::Text2dBounds};
use rand::Rng;

use crate::config::*;

// 初始化，空白面板，在随机的两个位置生成 2
pub fn init_cell_value_save() -> Vec<Vec<u32>> {
    let mut cell_value_save_temp: Vec<Vec<u32>> = Vec::new();
    let mut pos_save: Vec<Vec<usize>> = Vec::new();
    for i in 0..CELL_SIDE_NUM {
        let mut cell_value_save_temp_row: Vec<u32> = Vec::new();
        for j in 0..CELL_SIDE_NUM {
            cell_value_save_temp_row.push(0);
            let temp_pos = vec![i as usize, j as usize];
            pos_save.push(temp_pos);
        }
        cell_value_save_temp.push(cell_value_save_temp_row);
    }

    let mut index = rand::thread_rng().gen_range(0..16) as usize;
    cell_value_save_temp[pos_save[index][0]][pos_save[index][1]] = 2;
    pos_save.remove(index);
    index = rand::thread_rng().gen_range(0..15) as usize;
    cell_value_save_temp[pos_save[index][0]][pos_save[index][1]] = 2;
    cell_value_save_temp
}

// 判断游戏胜负
pub fn check_result(save_value: &mut CellValueSave) -> VictoryOrDefeat {
    // 有2048判断玩家胜利
    for i in 0..CELL_SIDE_NUM as usize {
        for j in 0..CELL_SIDE_NUM as usize {
            if save_value.value_save[i][j] == 2048 {
                return VictoryOrDefeat::Victory;
            }
        }
    }

    // 未胜利，有空位，游戏继续
    for i in 0..CELL_SIDE_NUM as usize {
        for j in 0..CELL_SIDE_NUM as usize {
            if save_value.value_save[i][j] == 0 {
                return VictoryOrDefeat::None;
            }
        }
    }

    // 没有空位，但是有可合并的点，游戏继续
    for i in 0..CELL_SIDE_NUM as usize - 1 {
        for j in 0..CELL_SIDE_NUM as usize - 1 {
            if save_value.value_save[i][j] == save_value.value_save[i + 1][j]
                || save_value.value_save[i][j] == save_value.value_save[i][j + 1]
            {
                return VictoryOrDefeat::None;
            }
        }
    }

    // 以上都不满足，无法再移动，玩家输
    VictoryOrDefeat::Defeat
}

// 判断是否有空位
pub fn have_empty(save_value: &mut [Vec<u32>]) -> bool {
    for item in save_value.iter().take(CELL_SIDE_NUM as usize) {
        for cell in item.iter().take(CELL_SIDE_NUM as usize) {
            if cell == &0 {
                return true;
            }
        }
    }
    false
}

// 根据数字更新UI
pub fn sync_data_to_display_system(
    ev_change: EventReader<DateChangeEvent>,
    asset_server: Res<AssetServer>,
    mut cell_value_save: ResMut<CellValueSave>,
    mut text_query: Query<&mut Text, With<CellValue>>,
    mut score_query: Query<&mut Text, Without<CellValue>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state: ResMut<NextState<VictoryOrDefeat>>,
) {
    if ev_change.is_empty() {
        return;
    }

    score_query.single_mut().sections[1].value = cell_value_save.score.to_string();

    let side_length: f32 =
        (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let mut text_style = TextStyle {
        font,
        font_size: side_length / 2.0,
        color: COLOR_BROWN,
    };

    for (i, mut text) in text_query.iter_mut().enumerate() {
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
        materials.set_untracked(
            cell_value_save.cell_back_ground[i],
            ColorMaterial::from(cell_color(cell_value_save.value_save[i / 4][i % 4])),
        );
    }

    let result = check_result(&mut cell_value_save);
    match result {
        VictoryOrDefeat::Victory => {
            app_state.set(VictoryOrDefeat::Victory);
        }
        VictoryOrDefeat::Defeat => {
            app_state.set(VictoryOrDefeat::Defeat);
        }
        VictoryOrDefeat::None => {}
    };
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_move: EventWriter<MoveEvent>,
) {
    let mut moved = MoveDirection::None;
    if keyboard_input.just_pressed(KeyCode::Up) {
        moved = MoveDirection::Up;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        moved = MoveDirection::Down;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        moved = MoveDirection::Right;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        moved = MoveDirection::Left;
    }

    println!("ev_move send");
    ev_move.send(MoveEvent(moved));
}

// 处理触摸事件
pub fn touch_events(
    mut touch_evr: EventReader<TouchInput>,
    mut start_pos: Local<Option<Vec2>>,
    mut ev_move: EventWriter<MoveEvent>,

) {
    use bevy::input::touch::TouchPhase;
    for ev in touch_evr.iter() {
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                *start_pos = Some(ev.position);
            }
            TouchPhase::Moved => {

            }
            TouchPhase::Ended => {
                if let Some(start_pos) = *start_pos {
                    let delta = ev.position - start_pos;

                    
                    if delta.length() > 50.0 {
                        if delta.x.abs() > delta.y.abs() {
                            if delta.x > 0.0 {
                                ev_move.send(MoveEvent(MoveDirection::Right));
                            } else {
                                ev_move.send(MoveEvent(MoveDirection::Left));
                            }
                        } else {
                            if delta.y > 0.0 {
                                ev_move.send(MoveEvent(MoveDirection::Down));
                            } else {
                                ev_move.send(MoveEvent(MoveDirection::Up));
                            }
                        }
                    }
                }
                *start_pos = None;
            }
            TouchPhase::Cancelled => {
                *start_pos = None;
            }
        }
    }
}

pub fn move_handler_system(
    mut ev_move: EventReader<MoveEvent>,
    mut ev_change: EventWriter<DateChangeEvent>,
    mut save_value: ResMut<CellValueSave>,
) {
    // 判断是否要新生成 2或4 的flag
    let is_move;
    if ev_move.is_empty() {
        return;
    }

    if let Some(direction) = ev_move.iter().next() {
        match direction.0 {
            MoveDirection::None => return,
            MoveDirection::Right => is_move = to_right(&mut save_value),
            MoveDirection::Left => is_move = to_left(&mut save_value),
            MoveDirection::Up => is_move = to_up(&mut save_value),
            MoveDirection::Down => is_move = to_down(&mut save_value),
        }

        let have_empty = have_empty(&mut save_value.value_save);

        // 在空位生成新的数
        if is_move && have_empty {
            let mut temp: u32 = rand::thread_rng().gen_range(0..10) as u32;
            if temp > 0 {
                temp = 2;
            } else {
                temp = 4;
            }
            let mut pos_save: Vec<Vec<usize>> = Vec::new();
            for i in 0..CELL_SIDE_NUM as usize {
                for j in 0..CELL_SIDE_NUM as usize {
                    if save_value.value_save[i][j] == 0 {
                        let pos = vec![i, j];
                        pos_save.push(pos);
                    }
                }
            }
            let index = rand::thread_rng().gen_range(0..pos_save.len());
            save_value.value_save[pos_save[index][0]][pos_save[index][1]] = temp;
        }

        ev_change.send(DateChangeEvent);
    }
}

// 向右移动
pub fn to_right(save_value: &mut CellValueSave) -> bool {
    let mut is_move = false;
    for i in 0..CELL_SIDE_NUM as usize {
        for j in (0..CELL_SIDE_NUM as usize).rev() {
            if save_value.value_save[i][j] == 0 {
                continue;
            }
            for k in (0..j).rev() {
                if save_value.value_save[i][k] == 0 {
                    continue;
                }
                if save_value.value_save[i][k] != save_value.value_save[i][j] {
                    break;
                } else {
                    save_value.value_save[i][j] += save_value.value_save[i][k];
                    save_value.score += save_value.value_save[i][j];
                    save_value.value_save[i][k] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    for i in 0..CELL_SIDE_NUM as usize {
        for j in (0..CELL_SIDE_NUM as usize).rev() {
            if save_value.value_save[i][j] != 0 {
                continue;
            }
            for k in (0..j).rev() {
                if save_value.value_save[i][k] == 0 {
                    continue;
                } else {
                    save_value.value_save[i][j] = save_value.value_save[i][k];
                    save_value.value_save[i][k] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    is_move
}

// 向左移动
pub fn to_left(save_value: &mut CellValueSave) -> bool {
    let mut is_move = false;
    for i in 0..CELL_SIDE_NUM as usize {
        for j in 0..CELL_SIDE_NUM as usize {
            if save_value.value_save[i][j] == 0 {
                continue;
            }
            for k in j + 1..CELL_SIDE_NUM as usize {
                if save_value.value_save[i][k] == 0 {
                    continue;
                }
                if save_value.value_save[i][k] != save_value.value_save[i][j] {
                    break;
                } else {
                    save_value.value_save[i][j] += save_value.value_save[i][k];
                    save_value.score += save_value.value_save[i][j];
                    save_value.value_save[i][k] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    for i in 0..CELL_SIDE_NUM as usize {
        for j in 0..CELL_SIDE_NUM as usize {
            if save_value.value_save[i][j] != 0 {
                continue;
            }
            for k in j + 1..CELL_SIDE_NUM as usize {
                if save_value.value_save[i][k] == 0 {
                    continue;
                } else {
                    save_value.value_save[i][j] = save_value.value_save[i][k];
                    save_value.value_save[i][k] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    is_move
}

// 向上移动
pub fn to_up(save_value: &mut CellValueSave) -> bool {
    let mut is_move = false;
    for i in 0..CELL_SIDE_NUM as usize {
        for j in 0..CELL_SIDE_NUM as usize {
            if save_value.value_save[j][i] == 0 {
                continue;
            }
            for k in j + 1..CELL_SIDE_NUM as usize {
                if save_value.value_save[k][i] == 0 {
                    continue;
                }
                if save_value.value_save[k][i] != save_value.value_save[j][i] {
                    break;
                } else {
                    save_value.value_save[j][i] += save_value.value_save[k][i];
                    save_value.score += save_value.value_save[j][i];
                    save_value.value_save[k][i] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    for i in 0..CELL_SIDE_NUM as usize {
        for j in 0..CELL_SIDE_NUM as usize {
            if save_value.value_save[j][i] != 0 {
                continue;
            }
            for k in j + 1..CELL_SIDE_NUM as usize {
                if save_value.value_save[k][i] == 0 {
                    continue;
                } else {
                    save_value.value_save[j][i] = save_value.value_save[k][i];
                    save_value.value_save[k][i] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    is_move
}

// 向下移动
pub fn to_down(save_value: &mut CellValueSave) -> bool {
    let mut is_move = false;
    for i in 0..CELL_SIDE_NUM as usize {
        for j in (0..CELL_SIDE_NUM as usize).rev() {
            if save_value.value_save[j][i] == 0 {
                continue;
            }
            for k in (0..j).rev() {
                if save_value.value_save[k][i] == 0 {
                    continue;
                }
                if save_value.value_save[k][i] != save_value.value_save[j][i] {
                    break;
                } else {
                    save_value.value_save[j][i] += save_value.value_save[k][i];
                    save_value.score += save_value.value_save[j][i];
                    save_value.value_save[k][i] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    for i in 0..CELL_SIDE_NUM as usize {
        for j in (0..CELL_SIDE_NUM as usize).rev() {
            if save_value.value_save[j][i] != 0 {
                continue;
            }
            for k in (0..j).rev() {
                if save_value.value_save[k][i] == 0 {
                    continue;
                } else {
                    save_value.value_save[j][i] = save_value.value_save[k][i];
                    save_value.value_save[k][i] = 0;
                    is_move = true;
                    break;
                }
            }
        }
    }

    is_move
}

fn cell_color(cell_value: u32) -> bevy::render::color::Color {
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

pub fn defeat_fn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cell_value_save: ResMut<CellValueSave>,
    entities: Query<Entity, Without<Camera>>,
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
        text: Text::from_section(text, text_style).with_alignment(TextAlignment::Center),
        text_2d_bounds: Text2dBounds { size: box_size },
        ..default()
    });
}

pub fn victory_function(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cell_value_save: ResMut<CellValueSave>,
    entities: Query<Entity, Without<Camera>>,
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
        text: Text::from_section(text, text_style).with_alignment(TextAlignment::Center),
        text_2d_bounds: Text2dBounds { size: box_size },
        ..default()
    });
}
