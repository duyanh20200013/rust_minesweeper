use rand::Rng;
use rusty_engine::prelude::*;

const BOMB_COUNT_NAME_LABEL: &[&'static str] = &[
    "cell", "one", "two", "three", "four", "five", "six", "seven", "eight",
];

#[derive(Clone, Copy)]
pub struct Cell {
    pub surrounds: u8,
    pub bomb: bool,
    pub flag: bool,
    pub revealed: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            surrounds: 0,
            bomb: false,
            flag: false,
            revealed: false,
        }
    }
}

pub struct Minesweeper {
    pub playing: bool,
    pub grid: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Default for Minesweeper {
    fn default() -> Self {
        Minesweeper::new(10, 10, 10)
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, number_of_mines: usize) -> Self {
        let playing = true;
        let mut grid = vec![vec![Cell::default(); width]; height];
        let mut bombs = vec![];

        let mut rng = rand::thread_rng();
        let mut count = 0;

        while number_of_mines != count {
            let y = rng.gen_range(0..height);
            let x = rng.gen_range(0..width);

            if grid[y][x].bomb == false {
                grid[y][x].bomb = true;
                bombs.push((x, y));
                count += 1;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if width as isize > nx && nx >= 0 && height as isize > ny && ny >= 0 {
                            grid[ny as usize][nx as usize].surrounds += 1;
                        }
                    }
                }
            }
        }

        Minesweeper {
            playing,
            grid,
            width,
            height,
        }
    }

    pub fn open(&mut self, x: usize, y: usize) {
        if !self.grid[y][x].flag && self.playing {
            match self.grid[y][x].bomb {
                true => {
                    self.grid[y][x].revealed = true;
                    self.playing = false;
                }
                false => {
                    if self.grid[y][x].surrounds != 0 {
                        self.grid[y][x].revealed = true;
                    } else {
                        self.open_empty(x, y)
                    }
                }
            }
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        if self.playing {
            self.grid[y][x].flag = !self.grid[y][x].flag;
        }
    }

    fn open_empty(&mut self, x: usize, y: usize) {
        if self.grid[y][x].revealed == true {
            return;
        }
        if self.grid[y][x].surrounds != 0 {
            self.grid[y][x].revealed = true;
            return;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if self.width as isize > nx && nx >= 0 && self.height as isize > ny && ny >= 0 {
                    self.grid[y][x].revealed = true;
                    self.open_empty(nx as usize, ny as usize);
                }
            }
        }
    }
}

struct GameState {
    height: usize,
    width: usize,
    count_flag: usize,
    ms: Minesweeper,
    start: bool,
}

fn main() {
    // Create a game
    let mut game = Game::new();
    let game_new = game.add_text("game_text", "WELCOME TO MINESWEEPER");
    game_new.font_size = 50.0;
    let level = game.add_text(
        "level",
        "Press E to choose easy level or press D to choose difficult level ",
    );
    level.font_size = 30.0;
    level.translation.y = -100.0;
    let initial_game_state = GameState {
        height: 8 as usize,
        width: 8 as usize,
        count_flag: 10 as usize,
        ms: Minesweeper::new(8, 8, 10),
        start: false,
    };

    game.add_logic(game_logic);
    game.run(initial_game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.ms.playing {
        let level = engine.add_text(
            "level",
            "Press E to choose easy level or press D to choose difficult level ",
        );
        level.font_size = 30.0;
        level.translation.y = -100.0;
        level.layer = 20.0;
        game_state.start = false;
    }
    initialization(engine, game_state);
    if game_state.start {
        if engine.mouse_state.just_pressed(MouseButton::Right) {
            if let Some(location) = engine.mouse_state.location() {
                for i in 0..game_state.height {
                    for j in 0..game_state.width {
                        let cell = engine
                            .sprites
                            .get_mut(&(i * 100 + j).to_string()[..])
                            .unwrap();
                        if location.x - cell.translation.x <= 16.0
                            && location.x - cell.translation.x > -16.0
                            && location.y - cell.translation.y <= 16.0
                            && location.y - cell.translation.y > -16.0
                        {
                            game_state.ms.flag(i, j);
                            let label_name = format!("flag{}", i * 100 + j);
                            let flag = engine.sprites.get_mut(&label_name[..]).unwrap();
                            if game_state.ms.grid[j][i].flag {
                                flag.layer = 13.0;
                            } else {
                                flag.layer = 11.0;
                            }
                        }
                    }
                }
            }
        }
        if engine.mouse_state.just_pressed(MouseButton::Left) {
            if let Some(location) = engine.mouse_state.location() {
                for i in 0..game_state.height {
                    for j in 0..game_state.width {
                        let cell = engine
                            .sprites
                            .get_mut(&(i * 100 + j).to_string()[..])
                            .unwrap();
                        if location.x - cell.translation.x <= 16.0
                            && location.x - cell.translation.x > -16.0
                            && location.y - cell.translation.y <= 16.0
                            && location.y - cell.translation.y > -16.0
                        {
                            game_state.ms.open(i, j);
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for i in 0..game_state.height {
            for j in 0..game_state.width {
                if game_state.ms.grid[j][i].flag {
                    let label_name = format!("flag{}", i * 100 + j);
                    let flag = engine.sprites.get_mut(&label_name[..]).unwrap();
                    flag.layer = 13.0;
                } else if game_state.ms.grid[j][i].revealed {
                    count += 1;
                    if game_state.ms.grid[j][i].bomb {
                        let label_name = format!("bomb{}", i * 100 + j);
                        let bomb = engine.sprites.get_mut(&label_name[..]).unwrap();
                        bomb.layer = 14.0;
                        let game_over = engine.add_text("game_text", "Game Over");
                        game_over.font_size = 128.0;
                        game_over.layer = 20.0;
                    } else {
                        let surr = game_state.ms.grid[j][i].surrounds;
                        let label_name =
                            format!("{}{}", BOMB_COUNT_NAME_LABEL[surr as usize], i * 100 + j);
                        let cell = engine.sprites.get_mut(&label_name[..]).unwrap();
                        cell.layer = 14.0;
                    }
                }
            }
        }
        if game_state.height * game_state.width - count == game_state.count_flag {
            game_state.ms.playing = false;
            let game_over = engine.add_text("game_text", "You Win");
            game_over.font_size = 128.0;
            game_over.layer = 20.0;
        }
    }
}

fn initialization(engine: &mut Engine, game_state: &mut GameState) {
    if (engine.keyboard_state.just_pressed(KeyCode::D)
        || engine.keyboard_state.just_pressed(KeyCode::E))
        && !game_state.start
    {
        engine.sprites.clear();
        engine.texts.clear();
        if engine.keyboard_state.just_pressed(KeyCode::D) && !game_state.start {
            game_state.height = 16;
            game_state.width = 16;
            game_state.count_flag = 40;
            game_state.start = true;
        }
        if engine.keyboard_state.just_pressed(KeyCode::E) && !game_state.start {
            game_state.height = 8;
            game_state.width = 8;
            game_state.count_flag = 10;
            game_state.start = true;
        }
        game_state.ms =
            Minesweeper::new(game_state.width, game_state.height, game_state.count_flag);
        for i in 0..(game_state.height) {
            for j in 0..(game_state.width) {
                let sprite = engine.add_sprite((i * 100 + j).to_string(), "img_out/cell.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 12.0;

                let label_name = format!("bomb{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/bomb.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 10.0;

                let label_name = format!("flag{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/flag.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 11.0;

                let label_name = format!("cell{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/open_cell.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 9.0;

                let label_name = format!("eight{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/8.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 8.0;

                let label_name = format!("seven{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/7.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 7.0;

                let label_name = format!("six{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/6.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 6.0;

                let label_name = format!("five{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/5.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 5.0;

                let label_name = format!("four{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/4.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 4.0;

                let label_name = format!("three{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/3.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 3.0;

                let label_name = format!("two{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/2.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 2.0;

                let label_name = format!("one{}", i * 100 + j);
                let sprite = engine.add_sprite(label_name, "img_out/1.png");
                sprite.scale = 2.0;
                sprite.translation.x = i as f32 * 32.0 + (game_state.height - 1) as f32 * -16.0;
                sprite.translation.y = j as f32 * 32.0 + (game_state.width - 1) as f32 * -16.0;
                sprite.layer = 1.0;
            }
        }
    }
}
