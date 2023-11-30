use std::{collections::HashSet, iter::FlatMap, fmt::{Write, write, Display}};

use rand::{thread_rng, Rng};

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(usize)
}

#[derive(Debug)]
pub struct Minesweeper{
    // Maybe change width and height for a L x L type , instead of using 2 variables 
    width: usize,
    height: usize,

    open_field: HashSet<Position>,
    mines: HashSet<Position>,
    flags: HashSet<Position>,

}

impl Display for Minesweeper{
    fn fmt(&self , f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        for y in 0..self.height{
            for x in 0..self.width{
                let pos = (x,y);
                if !self.open_field.contains(&pos){
                    f.write_str("🟪 ")?;
                }else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    write!(f," {} ", self.neighboring_mines(pos))?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize , height: usize, mine_count: usize) -> Minesweeper{
        Minesweeper {
            width,
            height,
            open_field: HashSet::new(), 
            mines: Self::generate_mines(width,height,mine_count),
            flags: HashSet::new(), 
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

    pub fn iter_neighbors(&self, (x,y): Position) -> impl Iterator<Item = Position>{
        let width = self.width;
        let height = self.height;
        (x.min(1) - 1 ..= (y + 1).min(width- 1))
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

    pub fn open(&mut self , position: Position) -> OpenResult {
        self.open_field.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine{
            OpenResult::Mine
        } else {
            OpenResult::NoMine(0)
        }
        
    }

}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;
    
    #[test]
    fn test(){
        let mut ms: Minesweeper = Minesweeper::new(10,10,5);
        ms.open((5,5));

        println!("{}",ms);
    }
}
