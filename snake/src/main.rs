use std::collections::LinkedList;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, ButtonEvent, Button, ButtonState};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::keyboard::Key;

use rand::{thread_rng, Rng};


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
    grid: i32,
    snake: Snake,
    food: (i32, i32),
    state: GameState,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        
        // Clearn screen
        self.gl.draw(args.viewport(), |_c:graphics::Context, gl| {
            graphics::clear(GREEN, gl);                        
        });

        // Draw food
        self.gl.draw(args.viewport(), |c:graphics::Context, gl| {
            let transform = c.transform;
            let square = graphics::rectangle::square((self.food.0 * self.grid) as f64, (self.food.1 * self.grid) as f64, self.grid as f64);  

            graphics::rectangle(BLUE, square, transform, gl)                      
        });

        self.snake.render(&mut self.gl, args, &self.grid);
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.state == GameState::Playing {
            let butt = self.snake.update();
    
            self.snake.check_collisions(&mut self.state);
    
            if self.snake.get_head() == &self.food {
                let mut rng = thread_rng();
                let new_pos = (rng.gen_range(0..20), rng.gen_range(0..20));
                self.food = new_pos;
                self.snake.body.push_back(butt);
            }
        }
        else {
            return
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.direction.clone();
        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }
}


struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction
}
impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, grid: &i32) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        
        gl.draw(args.viewport(), |c:graphics::Context, gl| {
            let transform = c.transform;
            
            for bodypart in self.body.iter() {
                let square = graphics::rectangle::square((bodypart.0 * grid) as f64, (bodypart.1 * grid) as f64, *grid as f64);
                graphics::rectangle(RED, square, transform, gl)
            }
            
        }); 
    }

    fn update(&mut self) -> (i32, i32) {
        let mut new_head = self.get_head().clone();
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

    fn check_collisions (&mut self, state: &mut GameState) {
        let mut headless_body = self.body.clone();
        headless_body.pop_front();
        if (headless_body.contains(self.get_head())) | ((self.get_head().0 > 19) | (self.get_head().0 < 0) | (self.get_head().1 < 0) | (self.get_head().1 > 19))
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
    const GRID: i32 = 20;


    let mut window: Window = WindowSettings::new("Snake Game", [WINDOW_SIZE as f64, WINDOW_SIZE as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    // let head_size = (WINDOW_SIZE / GRID) as i32;
    

    let snake = Snake {
        body: LinkedList::from([(1, 0), (0, 0)]),
        direction: Direction::Right,
    };

    let mut rng = thread_rng();
    let initial_food: (i32, i32) = (rng.gen_range(0..GRID), rng.gen_range(0..GRID));

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: WINDOW_SIZE / GRID,
        snake, 
        food: initial_food,
        state: GameState::Playing,
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
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
    }
}