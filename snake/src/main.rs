use std::collections::LinkedList;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, ButtonEvent, Button, ButtonState};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::keyboard::Key;

use rand::{thread_rng, Rng};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;


// Assumes that window size will always be square

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    grid_size: i32,
    rows_and_columns: i32,
    snake: Snake,
    food: (i32, i32),
    state: GameState,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        
        // Clean screen
        self.gl.draw(args.viewport(), |_c:graphics::Context, gl| {
            graphics::clear(GREEN, gl);                        
        });

        // Draw food
        self.gl.draw(args.viewport(), |c:graphics::Context, gl| {
            let transform = c.transform;
            let square = graphics::rectangle::square((self.food.0 * self.grid_size) as f64, (self.food.1 * self.grid_size) as f64, self.grid_size as f64);  

            graphics::rectangle(BLUE, square, transform, gl)                      
        });

        self.snake.render(&mut self.gl, args, &self.grid_size);
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.state == GameState::Playing {
            let tail = self.snake.update();
    
            self.snake.check_collisions(&mut self.state, &self.rows_and_columns);
    
            if self.snake.get_head() == &self.food {
                let mut rng = thread_rng();
                let new_pos = (rng.gen_range(0..self.rows_and_columns), rng.gen_range(0..self.rows_and_columns));
                self.food = new_pos;
                self.snake.body.push_back(tail);
            }
        }
        else {
            return
        }
    }

    fn pressed(&mut self, btn: &Button) {
        self.snake.attempted_direction = match btn {
            &Button::Keyboard(Key::Up) if self.snake.direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if self.snake.direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if self.snake.direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if self.snake.direction != Direction::Left => Direction::Right,
            _ => self.snake.direction.clone(),
        }
    }
}


struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction,
    attempted_direction : Direction,
}
impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, grid_size: &i32) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        
        gl.draw(args.viewport(), |c:graphics::Context, gl| {
            let transform = c.transform;
            
            for bodypart in self.body.iter() {
                let square = graphics::rectangle::square((bodypart.0 * grid_size) as f64, (bodypart.1 * grid_size) as f64, *grid_size as f64);
                graphics::rectangle(RED, square, transform, gl)
            }
            
        }); 
    }

    fn update(&mut self) -> (i32, i32) {
        let mut new_head = self.get_head().clone();
        self.direction = self.attempted_direction.clone();
        match self.direction {
            Direction::Right => new_head.0 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }
        self.body.push_front(new_head);
        self.body.pop_back().unwrap()
    }

    fn get_head(&mut self) -> &(i32, i32) {
        self.body.front().unwrap()
    }

    fn check_collisions (&mut self, state: &mut GameState, rows_and_columns: &i32) {
        let mut headless_body = self.body.clone();
        headless_body.pop_front();
        if (headless_body.contains(self.get_head())) | ((self.get_head().0 > rows_and_columns-1) | (self.get_head().0 < 0) | (self.get_head().1 < 0) | (self.get_head().1 > rows_and_columns-1))
        {
            *state = GameState::GameEnd;
            println!("==========YOU LOST===========");
            println!("=====PRESS ESCAPE TO EXIT====");
        }
    }
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    GameEnd,
}

fn main() {
    let opengl = OpenGL::V3_2;
    const WINDOW_SIZE: i32 = 600;
    const ROWS_AND_COLUMNS: i32 = 40;


    let mut window: Window = WindowSettings::new("Snake Game", [WINDOW_SIZE as f64, WINDOW_SIZE as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();    

    let snake = Snake {
        body: LinkedList::from([(1, 0), (0, 0)]),
        direction: Direction::Right,
        attempted_direction: Direction::Right,
    };

    let mut rng = thread_rng();
    let initial_food: (i32, i32) = (rng.gen_range(0..ROWS_AND_COLUMNS), rng.gen_range(0..ROWS_AND_COLUMNS));

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid_size: WINDOW_SIZE / ROWS_AND_COLUMNS,
        rows_and_columns: ROWS_AND_COLUMNS,
        snake, 
        food: initial_food,
        state: GameState::Playing,
    };

    let (tx, rx) = mpsc::channel();

    let mut speed = 12;
    thread::spawn(move || {
        loop {
            let wait_thread = thread::spawn(||{thread::sleep(Duration::from_secs(10))});
            wait_thread.join().expect("thread panicked");
            speed += 1;
            tx.send(speed).unwrap();           
        }
    });

    // scheduler.join().expect("Scheduler panicked");

    let mut events = Events::new(EventSettings::new()).ups(speed);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                app.pressed(&k.button);
            }
        }
        let received = rx.try_recv();
        if let Ok(speed) = received {
            events.set_ups(speed);
        }
    }
}
