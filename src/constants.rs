//! 游戏中使用的常量定义

use piston_window::types::Color;

// 定义游戏颜色常量
pub const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0]; // 背景颜色
pub const SNAKE_COLOR: Color = [0.0, 0.0, 1.0, 1.0]; // 蛇身颜色
pub const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0]; // 食物颜色
pub const BUTTON_COLOR: Color = [0.2, 0.6, 0.2, 1.0]; // 按钮颜色
pub const BUTTON_TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0]; // 按钮文字颜色

// 游戏配置常量
pub const CELL_SIZE: f64 = 20.0; // 每个格子的大小
pub const UPDATE_INTERVAL_MS: u64 = 200; // 游戏更新间隔（毫秒）
