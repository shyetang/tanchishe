//! 游戏状态和逻辑实现

use piston_window::*;
use rand::random;
use std::time::{Duration, Instant};

use crate::constants::*;
use crate::snake::Snake;

/// 游戏状态枚举
#[derive(PartialEq)]
pub enum GameState {
    NotStarted, // 游戏未开始
    Running,    // 游戏运行中
    Paused,     // 游戏暂停
}

/// 游戏状态结构体
pub struct Game {
    pub snake: Snake,              // 蛇
    pub food: (i32, i32),          // 食物位置
    pub width: i32,                // 游戏区域宽度
    pub height: i32,               // 游戏区域高度
    pub game_over: bool,           // 游戏结束标志
    pub last_update: Instant,      // 上次更新时间
    pub update_interval: Duration, // 更新间隔
    pub state: GameState,          // 游戏状态
    pub start_button: [f64; 4],    // 开始按钮区域 [x, y, width, height]
    pub mouse_pos: [f64; 2],       // 存储最后的鼠标位置
}

impl Game {
    /// 创建新游戏
    pub fn new(width: i32, height: i32) -> Self {
        let snake = Snake::new();
        let food = (width / 2, height / 2); // 初始食物位置

        // 设置开始按钮位置和大小
        let button_width = 120.0;
        let button_height = 40.0;
        let button_x = (width as f64 * CELL_SIZE - button_width) / 2.0;
        let button_y = (height as f64 * CELL_SIZE) / 2.0 - 50.0;

        Game {
            snake,
            food,
            width,
            height,
            game_over: false,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(UPDATE_INTERVAL_MS), // 每200毫秒更新一次
            state: GameState::NotStarted,                               // 初始状态为未开始
            start_button: [button_x, button_y, button_width, button_height],
            mouse_pos: [0.0, 0.0], // 初始化鼠标位置
        }
    }

    /// 更新游戏状态
    pub fn update(&mut self) {
        // 如果游戏未开始或已暂停，则不更新
        if self.state != GameState::Running || self.game_over {
            return;
        }

        // 检查是否达到更新间隔
        if self.last_update.elapsed() < self.update_interval {
            return;
        }

        self.last_update = Instant::now();

        // 移动蛇
        if !self.snake.move_forward(self.width, self.height) {
            self.game_over = true; // 撞到自己则游戏结束
            return;
        }

        // 检查是否吃到食物
        let head = self.snake.body.last().unwrap();
        if *head == self.food {
            self.snake.grow(); // 蛇增长
                               // 随机生成新食物
            self.food = (
                (random::<f32>() * self.width as f32) as i32,
                (random::<f32>() * self.height as f32) as i32,
            );
        }
    }

    /// 处理键盘输入
    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Up => {
                if self.state == GameState::Running {
                    self.snake.change_direction((0, -1))
                }
            }
            Key::Down => {
                if self.state == GameState::Running {
                    self.snake.change_direction((0, 1))
                }
            }
            Key::Left => {
                if self.state == GameState::Running {
                    self.snake.change_direction((-1, 0))
                }
            }
            Key::Right => {
                if self.state == GameState::Running {
                    self.snake.change_direction((1, 0))
                }
            }
            Key::Space => {
                // 空格键暂停/恢复游戏
                if self.state == GameState::Running {
                    self.state = GameState::Paused;
                } else if self.state == GameState::Paused {
                    self.state = GameState::Running;
                    self.last_update = Instant::now(); // 重置更新时间
                }
            }
            _ => {}
        }
    }

    /// 处理鼠标点击
    pub fn mouse_pressed(&mut self, pos: [f64; 2]) {
        // 检查是否点击了开始按钮
        if pos[0] >= self.start_button[0]
            && pos[0] <= self.start_button[0] + self.start_button[2]
            && pos[1] >= self.start_button[1]
            && pos[1] <= self.start_button[1] + self.start_button[3]
        {
            // 如果游戏未开始或已结束，则重新开始游戏
            if self.state == GameState::NotStarted || self.game_over {
                // 如果游戏已结束，重置蛇和食物
                if self.game_over {
                    self.snake = Snake::new();
                    self.food = (self.width / 2, self.height / 2);
                    self.game_over = false;
                }

                self.state = GameState::Running;
                self.last_update = Instant::now(); // 重置更新时间
            }
        }
    }
}
