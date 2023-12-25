use std::{collections::HashSet, iter::FlatMap, fmt::{Write, write, Display}};
 
#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8)
}

#[derive(Debug)]
pub struct Minesweeper{
    // Maybe change width and height for a L x L type , instead of using 2 variables 
    width: usize,
    height: usize,

    open_field: HashSet<Position>,
    mines: HashSet<Position>,
    flags: HashSet<Position>,

    lost: bool,
}

impl Display for Minesweeper{
    fn fmt(&self , f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        for y in 0..self.height{
            for x in 0..self.width{
                let pos = (x,y);
                if !self.open_field.contains(&pos){
                    if self.lost && self.mines.contains(&pos){
                        f.write_str("💣 ")?;
                    }else if self.flags.contains(&pos){
                        f.write_str("🚩 ")?;
                    }else{
                        f.write_str("🟪 ")?;
                    }                    
                }else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighboring_mines(pos);
                    if mine_count > 0 {
                        write!(f," {} ", mine_count)?;
                    }else{
                        f.write_str("⬜ ")?;
                    }

                    
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn random_range(min: usize, max: usize) -> usize {
  let mut rng = thread_rng();

  rng.gen_range(min..max)
}

#[wasm_bindgen]
extern "C" {
#[wasm_bindgen(js_namespace = Math)]
fn random() -> f64;
}   

#[cfg(target_family = "wasm")]
pub fn random_range(min: usize, max: usize) -> usize {
  (random() * (max - min) as f64).floor() as usize + min
}


impl Minesweeper {
    pub fn new(width: usize , height: usize, mine_count: usize) -> Minesweeper{
        Minesweeper {
            width,
            height,
            open_field: HashSet::new(), 
            mines: Self::generate_mines(width,height,mine_count),
            flags: HashSet::new(),
            lost: false,
        }
    }

    fn generate_mines(width: usize , height: usize, mine_count: usize) -> HashSet<Position>{
        let mut mines = HashSet::new();

        
        while mines.len() < mine_count {

            let x = random_range(0, width);
            let y = random_range(0, height);

            mines.insert((x,y));
        }
        mines

    }


    pub fn iter_neighbors(&self, (x,y): Position) -> impl Iterator<Item = Position>{
        let width = self.width;
        let height = self.height;
        (x.max(1) - 1 ..= (x + 1).min(width- 1))
        .flat_map(move |i|{
            (y.min(1)-1..=(y+1).min(height - 1)).map(move |j| (i , j))
        })
        .filter(move |&pos| pos != (x,y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self
        .iter_neighbors(pos)
        .filter(|pos| self.mines.contains(pos))
        .count() as u8
    }

    pub fn open(&mut self , position: Position) -> Option<OpenResult> {
        if self.open_field.contains(&position) {
            let mine_count = self.neighboring_mines(position);
            let flag_count = self.iter_neighbors(position).filter(| neighbor| self.flags.contains(neighbor)).count() as u8;
            
            if mine_count == flag_count {
               for neighbor in self.iter_neighbors(position){
                if !self.flags.contains(&neighbor){
                    self.open(neighbor);
                }
               } 
            }
        }

        if self.lost ||  self.flags.contains(&position) {
            return None  ;
        }
        self.open_field.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine{
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(position);

            if mine_count == 0{
        
                for neighbor in self.iter_neighbors(position){
                    if !self.open_field.contains(&neighbor){
                        self.open(neighbor);
                    }
                }
            }
            Some(OpenResult::NoMine(mine_count))
        }
        
    }
    
    pub fn toggle_flag(&mut self , pos: Position) {
        if self.open_field.contains(&pos){
            return;
            
        }

        if self.flags.contains(&pos){
            self.flags.remove(&pos);
        }else{
            self.flags.insert(pos);
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::minesweeper::{self, Minesweeper};
    
    #[test]
    fn test(){
        let mut ms: Minesweeper = Minesweeper::new(10,10,5);
        ms.open((5,5));
        ms.toggle_flag((6,6));
        ms.open((6,6));

        println!("{}",ms);
    }
}
