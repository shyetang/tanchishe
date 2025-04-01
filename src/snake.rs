//! 蛇的相关逻辑实现

/// 蛇的结构体
pub struct Snake {
    pub body: Vec<(i32, i32)>, // 蛇身坐标数组
    pub direction: (i32, i32), // 当前移动方向
}

impl Snake {
    /// 创建新蛇
    pub fn new() -> Self {
        Snake {
            body: vec![(0, 0), (0, 1), (0, 2)], // 初始蛇身
            direction: (0, 1),                  // 初始方向向下
        }
    }

    /// 蛇向前移动
    pub fn move_forward(&mut self, width: i32, height: i32) -> bool {
        let head = self.body.last().unwrap();
        // 计算新头部位置，使用取模运算实现边界穿越
        let new_head = (
            (head.0 + self.direction.0).rem_euclid(width),
            (head.1 + self.direction.1).rem_euclid(height),
        );

        // 检查是否撞到自己
        if self.body.contains(&new_head) {
            return false;
        }

        // 移动蛇身：添加新头部，移除尾部
        self.body.push(new_head);
        self.body.remove(0);
        true
    }

    /// 改变蛇的移动方向
    pub fn change_direction(&mut self, new_dir: (i32, i32)) {
        // 防止直接反向移动
        if (self.direction.0 + new_dir.0) != 0 || (self.direction.1 + new_dir.1) != 0 {
            self.direction = new_dir;
        }
    }

    /// 蛇吃到食物后增长
    pub fn grow(&mut self) {
        let tail = self.body[0];
        self.body.insert(0, tail); // 在尾部添加一节
    }
}
