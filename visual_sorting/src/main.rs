// The standard template taken from the git repo ggez : https://github.com/ggez/ggez
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
mod sorters;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("visual_sorter", "Jonathan Blomlöf")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let visual_sorter = VisualSorter::new(&mut ctx);

    let mut arr: [isize; 13];

    arr = [1,-3,4, 15, 2, -300, 30, 400, 51, 21, 20, 21, 15];
    sorters::insert_sort(&mut arr);
    println!("\n");
    print!("insert sort:\t");
    for i in arr {
        print!("{}, ", i);   
    }

    arr = [1,-3,4, 15, 2, -300, 30, 400, 51, 21, 20, 21, 15];
    sorters::selection_sort(&mut arr);
    println!("\n");
    print!("selection sort:\t");
    for i in arr {
        print!("{}, ", i);   
    }

    arr = [1,-3, 4, 15, 2, -300, 30, 400, 51, 21, 20, 21, 15];
    sorters::merge_sort(&mut arr);
    println!("\n");
    print!("merge sort:\t");
    for i in arr {
        print!("{}, ", i);   
    }

    arr = [1,-3, 4, 15, 2, -300, 30, 400, 51, 21, 20, 21, 15];
    sorters::yeet_sort(&mut arr);
    println!("\n");
    print!("yeet sort:\t");
    for i in arr {
        print!("{}, ", i);   
    }
    
    let mut _arr = [5,3,7,1,2,-2,-3,-6];

    let steps_done = sorters::merge_sort(&mut _arr);
    println!("\n");
    for i in 0..steps_done.len() {
        if i % _arr.len() == 0 {
            println!("");
        }
        print!("{}, ", steps_done.get(i).unwrap());
        
    }

    println!("");
    // Run!
    event::run(ctx, event_loop, visual_sorter);
}

struct VisualSorter {
    // Your state here...
}

impl VisualSorter {
    pub fn new(_ctx: &mut Context) -> VisualSorter {
        // Load/create resources such as images here.
        VisualSorter {
            // ...
        }
    }
}

impl EventHandler for VisualSorter {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}