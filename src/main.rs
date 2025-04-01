//! 贪吃蛇游戏主入口

mod constants;
mod game;
mod snake;
mod ui;

use piston_window::*;

use crate::constants::*;
use crate::game::Game;
use crate::ui::draw_game;
use crate::ui::load_font;

fn main() {
    // 初始化游戏区域
    let (width, height) = (30, 30);
    let mut game = Game::new(width, height);

    // 创建游戏窗口
    let mut window: PistonWindow = WindowSettings::new(
        "贪吃蛇",
        [
            width as u32 * CELL_SIZE as u32,
            height as u32 * CELL_SIZE as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    // 尝试加载字体
    let mut glyphs = load_font(&mut window);

    // 主游戏循环
    while let Some(e) = window.next() {
        // 处理键盘输入
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        // 处理鼠标移动
        if let Some(pos) = e.mouse_cursor_args() {
            game.mouse_pos = pos;
        }

        // 处理鼠标点击
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            game.mouse_pressed(game.mouse_pos);
        }

        // 绘制游戏
        window.draw_2d(&e, |c, g, device| {
            draw_game(&game, c, g, &mut glyphs);

            // 更新字体纹理
            if let Some(ref mut g_cache) = glyphs {
                g_cache.factory.encoder.flush(device);
            }
        });

        // 更新游戏状态
        e.update(|_arg| {
            game.update();
        });
    }
}
