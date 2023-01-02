
use wasm_bindgen::prelude::*;           // JS与Rust交互的包
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SNAKE_INIT_SIZE: usize = 3;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern {
    // 导入JS的random函数
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,        // 总格子数
    snake: Snake,
    reward_cell: Option<usize>, // 蛋
    next_cell: Option<SnakeCell>,            // 用来判断当用户第一次控制方向后，后面就一直往该方向走，直到再次转向
    status: Option<GameStatus>,
}

// 蛇的身体格子
#[derive(Copy, Clone, PartialEq)]       // PartialEq用来判断控制方向时不能返回自己的身体和能吃到蛋
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

// 这个枚举是从前端传进来的
#[wasm_bindgen]
#[derive(PartialOrd, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

// 游戏状态
#[wasm_bindgen]
#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum GameStatus {
    WON,
    LOSE,
    PLAYED,
}


#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        // 初始化时蛇共3格
        let snake = Snake::new(snake_index, SNAKE_INIT_SIZE);
        Self {
            width,
            size: width * width,
            reward_cell: Some(World::gen_reward_cell(width * width, &snake.body)),
            snake,
            next_cell: None,
            status: None,
        }
    }

    // 返回width给前端，用来设定画布的宽
    pub fn get_width(&self) -> usize {
        self.width
    }

    // 给前端返回蛇头的初始位置
    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    // 假设蛇的位置是[0][1][2]
    // 第二次渲染 [0]无论方向怎么变，[1]都是第一次的[0], [2]是第一次渲染的[1]
    pub fn update_snake(&mut self) {
        // let snake_head_index = self.snake_head_index();
        //
        // // 取模，防止蛇撞到画布然后崩溃
        // // self.snake.body[0].0 = (snake_head_index + 1) % self.size;
        // let (row, col) = self.index_to_cell(snake_head_index);
        // let (row, col) = match self.snake.direction {
        //     Direction::LEFT => (row, (col - 1) % self.width),
        //     Direction::RIGHT => (row, (col + 1) % self.width),
        //     Direction::UP => ((row - 1) % self.width, col),
        //     Direction::DOWN => ((row + 1) % self.width, col),
        // };
        // let next_index = self.cell_to_index(row, col);
        // self.set_snake_head(next_index);

        // 头和身一起变化的版本
        let temp = self.snake.body.clone();

        // 使用Option来提高性能，不用重复调用gen_next_snake_cell
        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            },
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }

        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i-1].0);
        }

        // 蛇头碰到蛇身，报错
        if self.snake.body[1..len].contains(&self.snake.body[0]) {
            self.status = Some(GameStatus::LOSE);
        }

        if self.reward_cell == Some(self.snake_head_index()) {
            // 蛇身长度小于总格子数游戏才能继续
            if self.snake_length() < self.size {
                // 蛋被吃掉后，重新生成一个蛋
                self.reward_cell = Some(World::gen_reward_cell(self.size, &self.snake.body));

                // 把蛋放到蛇头后面的位置，即蛇身的第一个位置
                self.snake.body.push(SnakeCell(self.snake.body[1].0));
            } else {
                self.reward_cell = None;

                self.status = Some(GameStatus::WON);
            }
        }
    }

    // 寻找蛇身的下一个位置
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;
        // 通过边界的方式更新下一个位置，提高性能。上面用的取余的方式性能不高
        return match direction {
            Direction::UP => {
                let border_hold = snake_index - (row * self.width);
                // 如果走到上边界，那么就从下面穿出来
                if snake_index == border_hold {
                    SnakeCell((self.size - self.width) + border_hold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            },
            Direction::DOWN => {
                let border_hold = snake_index + ((self.width - row) * self.width);
                if snake_index == border_hold {
                    SnakeCell(border_hold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_index + self.width)
                }
            },
            Direction::LEFT => {
                let border_hold = row * self.width;
                if snake_index == border_hold {
                    SnakeCell(border_hold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            },
            Direction::RIGHT => {
                let border_hold = (row + 1) * self.width;
                if snake_index + 1 == border_hold {
                    SnakeCell(border_hold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            },
        }
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        // 如果正在向左，不能立即向右
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.snake.direction = direction;
    }

    // 返回蛇头当前的行和列
    // fn index_to_cell(&self, index: usize) -> (usize, usize) {
    //     (index / self.width, index % self.width)
    // }

    // 传入坐标，返回在动态数组中的位置
    // fn cell_to_index(&self, row: usize, col: usize) -> usize {
    //     (row * self.width) + col
    // }

    // fn set_snake_head(&mut self, index: usize) {
    //     self.snake.body[0].0 = index;
    // }

    // 蛋不能在蛇身，第一个参数max是格子总数，用于随机位置生成
    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        reward_cell
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    // 返回蛇身长度
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::PLAYED)
    }

    // 用于返回状态
    pub fn get_game_status(&self) -> Option<GameStatus> {
        self.status
    }

    // 用于返回状态的具体信息
    pub fn get_game_status_info(&self) -> String {
        match self.status {
            Some(GameStatus::WON) => String::from("Won!"),
            Some(GameStatus::PLAYED) => String::from("You're playing!"),
            Some(GameStatus::LOSE) => String::from("You're Lose!"),
            None => String::from("None!"),
        }
    }
}

impl Snake {
    // 蛇的初始出生点
    pub fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }
        Self {
            body,
            direction: Direction::DOWN              // 默认向下
        }
    }
}