use macroquad::prelude::*; // Import common macroquad items
use std::collections::LinkedList;
extern crate rand; 
use rand::{thread_rng, Rng}; // Use the rand crate for more control if needed

const BLOCK_SIZE: f32 = 25.0; // Size of each block in pixels (use f32 for macroquad)
const WIDTH_BLOCKS: i32 = 24; // Game width in blocks
const HEIGHT_BLOCKS: i32 = 20; // Game height in blocks

const MOVE_PERIOD: f64 = 0.12; // Time between moves (controls speed)
const RESTART_DELAY: f64 = 1.5; // Delay before restarting after game over

// Colors defined using macroquad's Color struct
// Colors defined using macroquad's Color struct
// Define colors manually using Color::new(r, g, b, a) to avoid const fn float issues
const BACKGROUND_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0); // Black
const SNAKE_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);    // Green
const FOOD_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);     // Red
const GAMEOVER_COLOR: Color = Color::new(1.0, 0.0, 0.0, 0.6); // Semi-transparent red (This one was already fine)
const GAMEOVER_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0); // White

// --- Direction Enum ---
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// --- Snake Struct ---
#[derive(Debug)]
struct Snake {
    direction: Direction,
    body: LinkedList<(i32, i32)>, // Grid coordinates
    tail: Option<(i32, i32)>,      // For growth
}

impl Snake {
    fn new(x: i32, y: i32) -> Self {
        let mut body = LinkedList::new();
        body.push_back((x, y));
        body.push_back((x - 1, y));
        body.push_back((x - 2, y));
        Self {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // Helper to convert grid coords to pixel coords for drawing
    fn grid_to_pixel(coord: i32) -> f32 {
        coord as f32 * BLOCK_SIZE
    }

    fn draw(&self) {
        for &(x, y) in self.body.iter() {
            draw_rectangle(
                Self::grid_to_pixel(x),
                Self::grid_to_pixel(y),
                BLOCK_SIZE,
                BLOCK_SIZE,
                SNAKE_COLOR,
            );
        }
    }

    fn head_position(&self) -> (i32, i32) {
        *self.body.front().unwrap() // Snake always has a head
    }

    fn move_forward(&mut self, dir_change: Option<Direction>) {
        if let Some(new_dir) = dir_change {
            if new_dir.opposite() != self.direction {
                self.direction = new_dir;
            }
        }

        let (head_x, head_y) = self.head_position();
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        self.body.push_front(new_head);
        self.tail = self.body.pop_back(); // Returns Option, which is what we want
    }

    fn next_head(&self, dir_change: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let mut current_dir = self.direction;
        if let Some(new_dir) = dir_change {
            if new_dir.opposite() != self.direction {
                current_dir = new_dir;
            }
        }
        match current_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    fn restore_tail(&mut self) {
        if let Some(t) = self.tail {
            self.body.push_back(t);
        }
    }

    fn overlaps_body(&self, x: i32, y: i32) -> bool {
        // Check if the given coords overlap any body segment (including head)
        self.body.iter().any(|&(seg_x, seg_y)| seg_x == x && seg_y == y)
    }

    fn overlaps_tail_only(&self, x: i32, y: i32) -> bool {
        // Check if the given coords overlap any body segment *except* the head
        self.body.iter().skip(1).any(|&(seg_x, seg_y)| seg_x == x && seg_y == y)
    }
}

// --- Game Struct ---
struct Game {
    snake: Snake,
    food_pos: (i32, i32),
    score: u32,
    game_over: bool,
    time_since_move: f64,
    time_since_game_over: f64,
    paused: bool, // Optional: Add pause functionality
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            snake: Snake::new(WIDTH_BLOCKS / 2, HEIGHT_BLOCKS / 2),
            food_pos: (0, 0), // Placeholder, will be set
            score: 0,
            game_over: false,
            time_since_move: 0.0,
            time_since_game_over: 0.0,
            paused: false,
        };
        game.spawn_food(); // Initial food placement
        game
    }

    fn spawn_food(&mut self) {
        let mut rng = thread_rng();
        loop {
            let x = rng.gen_range(0..WIDTH_BLOCKS);
            let y = rng.gen_range(0..HEIGHT_BLOCKS);
            // Ensure food doesn't spawn on the snake
            if !self.snake.overlaps_body(x, y) {
                self.food_pos = (x, y);
                break;
            }
        }
    }

    fn handle_input(&mut self) {
        if self.game_over { return; } // Ignore game input when game over

        // Optional Pause Toggle
        if is_key_pressed(KeyCode::P) {
            self.paused = !self.paused;
        }
        if self.paused { return; } // Ignore game input when paused

        let mut new_dir = None;
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            new_dir = Some(Direction::Up);
        } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            new_dir = Some(Direction::Down);
        } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            new_dir = Some(Direction::Left);
        } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            new_dir = Some(Direction::Right);
        }

        if let Some(dir) = new_dir {
            // Allow changing direction immediately if valid
            if dir.opposite() != self.snake.direction {
                self.snake.direction = dir;
                // Optional: force an update immediately after direction change?
                // self.time_since_move = MOVE_PERIOD; // Uncomment to make turns instant
            }
        }
    }

    fn update(&mut self, dt: f64) {
        if self.game_over {
            self.time_since_game_over += dt;
            if self.time_since_game_over > RESTART_DELAY {
                // Restart the game check - can be triggered by input later
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    *self = Game::new(); // Reinitialize the game state
                }
            }
            return; // Don't update game logic if game over
        }

        if self.paused { return; } // Don't update if paused

        self.time_since_move += dt;

        if self.time_since_move >= MOVE_PERIOD {
            self.time_since_move -= MOVE_PERIOD; // Reset timer preserving overshoot

            // Calculate next position
            let (next_x, next_y) = self.snake.next_head(None); // Check based on current direction

            // Check for collisions
            if next_x < 0
                || next_x >= WIDTH_BLOCKS
                || next_y < 0
                || next_y >= HEIGHT_BLOCKS
                || self.snake.overlaps_tail_only(next_x, next_y)
            {
                self.game_over = true;
                self.time_since_game_over = 0.0;
                return; // End update on collision
            }

            // Move the snake
            self.snake.move_forward(None); // Pass None, direction already set

            // Check for eating food
            if self.snake.head_position() == self.food_pos {
                self.snake.restore_tail();
                self.score += 1;
                self.spawn_food();
            }
        }
    }

    fn draw(&self) {
        clear_background(BACKGROUND_COLOR);

        // Draw food
        draw_rectangle(
            Snake::grid_to_pixel(self.food_pos.0),
            Snake::grid_to_pixel(self.food_pos.1),
            BLOCK_SIZE,
            BLOCK_SIZE,
            FOOD_COLOR,
        );

        // Draw snake
        self.snake.draw();

        // Draw Score
        let score_text = format!("Score: {}", self.score);
        draw_text(&score_text, 10.0, 30.0, 30.0, WHITE);

        // Draw Game Over / Restart Message
        if self.game_over {
            // Dimming overlay
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                GAMEOVER_COLOR,
            );
            let text = "GAME OVER!";
            let text_size = measure_text(text, None, 60, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0 - text_size.height, // Position above center
                60.0,
                GAMEOVER_TEXT_COLOR,
            );

            if self.time_since_game_over > RESTART_DELAY {
                let restart_text = "Press SPACE to restart";
                let restart_size = measure_text(restart_text, None, 30, 1.0);
                draw_text(
                    restart_text,
                    screen_width() / 2.0 - restart_size.width / 2.0,
                    screen_height() / 2.0 + restart_size.height, // Position below center
                    30.0,
                    GAMEOVER_TEXT_COLOR,
                );
            }
        } else if self.paused {
            let text = "PAUSED (P)";
            let text_size = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0 - text_size.height / 2.0,
                50.0,
                WHITE,
            );
        }
    }
}

// --- Window Configuration ---
fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Snake".to_owned(),
        window_width: WIDTH_BLOCKS * BLOCK_SIZE as i32,
        window_height: HEIGHT_BLOCKS * BLOCK_SIZE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

// --- Main Function ---
#[macroquad::main(window_conf)]
async fn main() {
    // Seed macroquad's random number generator (optional but good practice)
    rand::rng().gen::<u32>();

    let mut game = Game::new();

    loop {
        game.handle_input();
        game.update(get_frame_time() as f64); // get_frame_time returns f32
        game.draw();

        next_frame().await // Wait for next frame
    }
}