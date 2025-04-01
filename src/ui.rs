//! 游戏界面绘制相关功能

use piston_window::*;
use std::path::PathBuf;

use crate::constants::*;
use crate::game::{Game, GameState};

/// 字体缓存类型
pub type FontCache = Glyphs;

/// 加载游戏字体
pub fn load_font(window: &mut PistonWindow) -> Option<FontCache> {
    // 尝试多种方式查找字体文件
    let mut font_path_found = false;
    let mut font_path = PathBuf::new();

    // 尝试加载系统中文字体（macOS系统字体）
    let system_fonts = vec![
        "/System/Library/Fonts/PingFang.ttc",         // macOS中文字体
        "/System/Library/Fonts/STHeiti Light.ttc",    // 另一个macOS中文字体
        "/System/Library/Fonts/AppleSDGothicNeo.ttc", // 韩文字体，但支持中文
        "assets/FiraSans-Regular.ttf",                // 原始字体作为备选
    ];

    // 尝试系统字体
    for system_font in &system_fonts {
        let try_path = PathBuf::from(system_font);
        if try_path.exists() {
            font_path = try_path;
            font_path_found = true;
            println!("找到系统字体: {:?}", font_path);
            break;
        }
    }

    // 如果系统字体都不存在，继续原来的查找逻辑
    if !font_path_found {
        // 方法1: 直接使用相对路径
        let direct_path = PathBuf::from("assets/FiraSans-Regular.ttf");
        if direct_path.exists() {
            font_path = direct_path;
            font_path_found = true;
            println!("找到字体文件(直接路径): {:?}", font_path);
        } else {
            println!("直接路径未找到字体: {:?}", direct_path);
        }

        // 方法2: 使用find_folder库
        if !font_path_found {
            if let Ok(assets) = find_folder::Search::ParentsThenKids(5, 5).for_folder("assets") {
                let try_path = assets.join("FiraSans-Regular.ttf");
                if try_path.exists() {
                    font_path = try_path;
                    font_path_found = true;
                    println!("找到字体文件(find_folder): {:?}", font_path);
                } else {
                    println!("find_folder找到assets文件夹但未找到字体: {:?}", try_path);
                }
            } else {
                println!("find_folder未找到assets文件夹");
            }
        }

        // 方法3: 使用当前可执行文件路径
        if !font_path_found {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    let try_path = exe_dir.join("assets/FiraSans-Regular.ttf");
                    if try_path.exists() {
                        font_path = try_path;
                        font_path_found = true;
                        println!("找到字体文件(可执行文件路径): {:?}", font_path);
                    } else {
                        println!("可执行文件路径未找到字体: {:?}", try_path);
                    }
                }
            }
        }

        // 方法4: 使用当前工作目录
        if !font_path_found {
            if let Ok(current_dir) = std::env::current_dir() {
                let try_path = current_dir.join("assets/FiraSans-Regular.ttf");
                if try_path.exists() {
                    font_path = try_path;
                    font_path_found = true;
                    println!("找到字体文件(当前工作目录): {:?}", font_path);
                } else {
                    println!("当前工作目录未找到字体: {:?}", try_path);

                    // 尝试向上一级目录查找
                    if let Some(parent_dir) = current_dir.parent() {
                        let try_path = parent_dir.join("assets/FiraSans-Regular.ttf");
                        if try_path.exists() {
                            font_path = try_path;
                            font_path_found = true;
                            println!("找到字体文件(上级目录): {:?}", font_path);
                        } else {
                            println!("上级目录未找到字体: {:?}", try_path);
                        }
                    }
                }
            }
        }
    }

    // 尝试加载字体
    if font_path_found {
        println!("尝试加载字体: {:?}", font_path);
        match window.load_font(&font_path) {
            Ok(g) => {
                println!("字体加载成功!");
                Some(g)
            }
            Err(e) => {
                println!("字体加载失败: {:?}", e);
                None
            }
        }
    } else {
        println!("警告: 无法找到字体文件，将使用简单的矩形代替文字");
        None
    }
}

/// 绘制游戏界面
pub fn draw_game(game: &Game, c: Context, g: &mut G2d, glyphs: &mut Option<FontCache>) {
    // 清空背景
    clear(BACK_COLOR, g);

    // 绘制食物
    rectangle(
        FOOD_COLOR,
        [
            game.food.0 as f64 * CELL_SIZE,
            game.food.1 as f64 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        ],
        c.transform,
        g,
    );

    // 绘制蛇
    for &(x, y) in &game.snake.body {
        rectangle(
            SNAKE_COLOR,
            [
                x as f64 * CELL_SIZE,
                y as f64 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
            ],
            c.transform,
            g,
        );
    }

    // 绘制开始按钮
    if game.state == GameState::NotStarted || game.game_over {
        rectangle(BUTTON_COLOR, game.start_button, c.transform, g);

        // 绘制按钮文字
        let button_text = if game.game_over {
            "重新开始"
        } else {
            "开始游戏"
        };
        let text_size = 20;
        let text_width = button_text.len() as f64 * (text_size as f64 * 0.5);
        let text_x = game.start_button[0] + (game.start_button[2] - text_width) / 2.0;
        let text_y = game.start_button[1] + game.start_button[3] / 2.0 + 7.0;

        // 使用字体渲染文字
        if let Some(ref mut g_cache) = glyphs {
            let transform = c.transform.trans(text_x, text_y);
            if let Err(e) = text::Text::new_color(BUTTON_TEXT_COLOR, text_size).draw(
                button_text,
                g_cache,
                &c.draw_state,
                transform,
                g,
            ) {
                println!("文本渲染错误: {}", e);
            }
        } else {
            // 如果字体加载失败，使用矩形代替文字
            rectangle(
                BUTTON_TEXT_COLOR,
                [
                    text_x,
                    text_y - text_size as f64,
                    text_width,
                    text_size as f64,
                ],
                c.transform,
                g,
            );
        }
    }

    // 如果游戏暂停，显示暂停提示
    if game.state == GameState::Paused {
        let pause_rect_width = 200.0;
        let pause_rect_height = 40.0;
        let pause_rect_x = (game.width as f64 * CELL_SIZE - pause_rect_width) / 2.0;
        let pause_rect_y = (game.height as f64 * CELL_SIZE - pause_rect_height) / 2.0;

        rectangle(
            [0.0, 0.0, 0.0, 0.5], // 半透明黑色
            [
                pause_rect_x,
                pause_rect_y,
                pause_rect_width,
                pause_rect_height,
            ],
            c.transform,
            g,
        );

        // 添加暂停文字
        let pause_text = "游戏暂停";
        let pause_text_size = 24;
        let pause_text_width = pause_text.len() as f64 * (pause_text_size as f64 * 0.5);
        let pause_text_x = pause_rect_x + (pause_rect_width - pause_text_width) / 2.0;
        let pause_text_y = pause_rect_y + pause_rect_height / 2.0 + 8.0;

        if let Some(ref mut g_cache) = glyphs {
            let transform = c.transform.trans(pause_text_x, pause_text_y);
            if let Err(e) = text::Text::new_color(BUTTON_TEXT_COLOR, pause_text_size).draw(
                pause_text,
                g_cache,
                &c.draw_state,
                transform,
                g,
            ) {
                println!("文本渲染错误: {}", e);
            }
        } else {
            // 如果字体加载失败，使用矩形代替文字
            rectangle(
                BUTTON_TEXT_COLOR,
                [
                    pause_text_x,
                    pause_text_y - pause_text_size as f64,
                    pause_text_width,
                    pause_text_size as f64,
                ],
                c.transform,
                g,
            );
        }
    }
}
