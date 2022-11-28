// The standard template taken from the git repo ggez : https://github.com/ggez/ggez
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use rand::prelude::SliceRandom;
use rand::thread_rng;
mod sorters_vec;

//constants from template
//modified to only allow one elemnt to take a unique value.
//eg. the values range from 0..vec.len()
//so it produces a smooth line.
const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const GRID_CELL_SIZE: usize = 5;

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_WIDTH as f32 * GRID_CELL_SIZE as f32,
    GRID_HEIGHT as f32 * GRID_CELL_SIZE as f32,
);

fn main() {
    // Make a Context.
    let context_builder = ggez::ContextBuilder::new("visual_sorter", "jblomlof")
        .window_setup(ggez::conf::WindowSetup::default().title("visual_sorter"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                .resizable(false),
        );
    let (mut ctx, event_loop) = context_builder
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let visual_sorter = VisualSorter::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, visual_sorter);
}

struct VisualSorter {
    // Your state here...
    sorting: [bool; 4],
    sort_step: usize,

    unsorted: [Vec<isize>; 4],

    solution: [Vec<Vec<isize>>; 4],

    sorted: [Vec<isize>; 4],
}

impl VisualSorter {
    pub fn new(_ctx: &mut Context) -> VisualSorter {
        // Load/create resources such as images here.
        let a1 = Self::generate_random_grid();
        let a2 = Self::generate_random_grid();
        let a3 = Self::generate_random_grid();
        let a4 = Self::generate_random_grid();
        let mut sort1 = a1.to_vec();
        let solution1 = sorters_vec::insert_sort(&mut sort1);
        let mut sort2 = a2.to_vec();
        let solution2 = sorters_vec::selection_sort(&mut sort2);
        let mut sort3 = a3.to_vec();
        let solution3 = sorters_vec::merge_sort(&mut sort3);
        let mut sort4 = a4.to_vec();
        let solution4 = sorters_vec::yeet_sort(&mut sort4);

        VisualSorter {
            // ...
            sorting: [true, false, false, false],
            sort_step: 0,
            unsorted: [a1, a2, a3, a4],
            solution: [solution1, solution2, solution3, solution4],
            sorted: [sort1, sort2, sort3, sort4],
        }
    }

    //copied and modified from template
    pub fn generate_random_grid() -> Vec<isize> {
        let mut random_gen = thread_rng();
        let mut values: Vec<isize> = (0..GRID_HEIGHT).map(|x| x as isize).collect();
        values.shuffle(&mut random_gen);
        values
    }
}

impl EventHandler for VisualSorter {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        //setting current so i can easily acces what algorithm it is. eg. 0 is insertion, 1 is selection, 2 is merge, 3 is yeet
        let current = {
            if self.sorting[0] {
                0
            } else if self.sorting[1] {
                1
            } else if self.sorting[2] {
                2
            } else {
                3
            }
        };
        if self.sort_step > self.solution[current].len() + 40 {
            // algorithm reached finish, go next.
            self.sorting[current] = false;
            if current != self.sorting.len() - 1 {
                self.sorting[current + 1] = true;
            } else {
                self.sorting[0] = true;
            } //start over but now we are on the next sortingalgorithm
            self.sort_step = 0;
        } else {
            // we are not done yet go to next stage
            self.sort_step += 1;
        }
        Ok(())
    }

    //everything involving drawing is inspired by GGEZ template snake game.
    //it was really hard figuring out what to do so it helped a lot to understand how you draw the most simple shape there is
    //but nothing was really taken, it just pointed out that canvas.draw is the way to go, but i guess you could say the whole canvas.draw() was taken
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        let current = {
            // same as in update
            if self.sorting[0] {
                0
            } else if self.sorting[1] {
                1
            } else if self.sorting[2] {
                2
            } else {
                3
            }
        };
        if self.sort_step < self.solution[current].len() {
            // sort_step is going to > max_index to allow for pauses to happen
            //this protects from index out of bounds.
            for _col in 0..GRID_WIDTH {
                let start_y_reversed = SCREEN_SIZE.1 as i32 // i want it to sort so y= 0 is at bottom
                    - GRID_CELL_SIZE as i32
                    - *self.solution[current]
                        .get(self.sort_step)
                        .unwrap()
                        .get(_col)
                        .unwrap() as i32
                        * GRID_CELL_SIZE as i32;
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            _col as i32 * GRID_CELL_SIZE as i32,
                            start_y_reversed,
                            GRID_CELL_SIZE as i32,
                            SCREEN_SIZE.1 as i32 - start_y_reversed, // making em rectangles with colour all the way from bottom to value
                        ))
                        .color(Color::from_rgb_u32(0xDe3163)),//den är cerise, inte rosa
                )
            }
            canvas.finish(ctx)?;
            ggez::timer::yield_now();
        }
        Ok(())
    }
}
