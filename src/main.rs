use std::collections::HashSet;

use rand::{thread_rng, Rng};

type Position = (usize, usize);
struct Minesweeper{
    // Maybe change width and height for a L x L type , instead of using 2 variables 
    width: usize,
    height: usize,

    open_field: HashSet<Position>,
    mines: HashSet<Position>,
    flags: HashSet<Position>,

}
impl Minesweeper {
    pub fn new(width: usize , height: usize, mine_count: usize) -> Minesweeper{
        Minesweeper {
            width,
            height,
            open_field: HashSet::new(), 
            mines: Self::generate_mines(width,height,mine_count),
            flags, 
        }
    }

    fn generate_mines(width: usize , height: usize, mine_count: usize) -> HashSet<Position>{
        let mut mines = HashSet::new();

        let mut rng = thread_rng();
        
        while mines.len() < mine_count {

            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            mines.insert((x,y));
        }
        mines

    }
}

fn main() {
    println!("Hello, world!");
}
