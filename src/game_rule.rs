
use rand::Rng;
use crate::config::*;

// 初始化，空白面板，在随机的两个位置生成 2
pub fn init_cell_value_save() -> Vec<Vec<u32>> {
	println!("Init_cell_value_save");
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
	return cell_value_save_temp;
}

// 判断游戏胜负
pub fn check_result(save_value: &mut CellValueSave) -> VictoryOrDefeat {
	// 有2048判断玩家胜利
	for i in 0..CELL_SIDE_NUM as usize {
		for j in 0..CELL_SIDE_NUM as usize {
			if save_value.value_save[i][j] == 2048 {
				return VictoryOrDefeat::VICTORY;
			}
		}
	}

	// 未胜利，有空位，游戏继续
	for i in 0..CELL_SIDE_NUM as usize {
		for j in 0..CELL_SIDE_NUM as usize {
			if save_value.value_save[i][j] == 0 {
				return VictoryOrDefeat::NONE;
			}
		}
	}

	// 没有空位，但是有可合并的点，游戏继续
	for i in 0..CELL_SIDE_NUM as usize-1 {
		for j in 0..CELL_SIDE_NUM as usize-1 {
			if save_value.value_save[i][j] == save_value.value_save[i + 1][j] ||
				save_value.value_save[i][j] == save_value.value_save[i][j + 1] {
				return VictoryOrDefeat::NONE;
			}
		}
	}

	// 以上都不满足，无法再移动，玩家输
	return VictoryOrDefeat::DEFEAT;
}

// 判断是否有空位
pub fn have_empty(save_value: &mut Vec<Vec<u32>>) -> bool {
	for i in 0..CELL_SIDE_NUM as usize {
		for j in 0..CELL_SIDE_NUM as usize {
			if save_value[i][j] == 0 {
				return true;
			}
		}
	}
	return false;
}

// 移动函数
pub fn move_value(direction: MoveDirection, save_value: &mut CellValueSave) {
	// 判断是否要新生成 2或4 的flag
	let is_move;

	match direction {
		MoveDirection::NONE => return ,
		MoveDirection::RIGHT => is_move = to_right(save_value),
		MoveDirection::LEFT => is_move = to_left(save_value),
		MoveDirection::UP => is_move = to_up(save_value),
		MoveDirection::DOWN => is_move = to_down(save_value),
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

	return ;
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

	return is_move;
}

// 向左移动
pub fn to_left(save_value: &mut CellValueSave) -> bool {

	let mut is_move = false;
	for i in 0..CELL_SIDE_NUM as usize {
		for j in 0..CELL_SIDE_NUM as usize as usize {
			if save_value.value_save[i][j] == 0 {
				continue;
			}
			for k in j+1..CELL_SIDE_NUM as usize {
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
			for k in j+1..CELL_SIDE_NUM as usize {
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

	return is_move;
}

// 向上移动
pub fn to_up(save_value: &mut CellValueSave) -> bool {

	let mut is_move = false;
	for i in 0..CELL_SIDE_NUM as usize {
		for j in 0..CELL_SIDE_NUM as usize {
			if save_value.value_save[j][i] == 0 {
				continue;
			}
			for k in j+1..CELL_SIDE_NUM as usize {
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
			for k in j+1..CELL_SIDE_NUM as usize {
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

	return is_move;
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

	return is_move;
}