use std::fmt::{self};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BCell {
    E, //empty
    X,
    O
}

impl fmt::Display for BCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BCell::E => write!(f, "{}", " "),
            BCell::X => write!(f, "X"),
            BCell::O => write!(f, "{}", "O"),
        }
    }
}


#[derive(Debug)]
pub struct Board 
{
    matrix: [[BCell; Self::GRID_SIZE]; Self::GRID_SIZE]
}

impl Board 
{
    const GRID_SIZE: usize = 3;

    fn check_bounds(&self, x: usize, y: usize) -> bool
    {
        x < Self::GRID_SIZE && y < Self::GRID_SIZE
    }

    pub fn new() -> Self 
    {
        Self { matrix: [[BCell::E; Self::GRID_SIZE]; Self::GRID_SIZE] }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<BCell>
    {
        if self.check_bounds(x, y)
        {
            Some(self.matrix[x][y])
        }
        else 
        {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, to_put: BCell) -> bool
    {
        match self.get(x, y)
        {
            Some(val) => {
                if val == BCell::E
                {
                    self.matrix[x][y] = to_put;
                    return true
                }
                else {
                    return  false
                }
            }
            None => panic!("Coordinates out of bounds: x={} y={}", x, y)
        }

        
    }

}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..Self::GRID_SIZE
        {
            for y in 0..Self::GRID_SIZE
            {
                match self.get(x, y)
                {
                    Some(v) => {
                        if y == Self::GRID_SIZE - 1
                        {
                            write!(f, "{:}", v)?
                        }
                        else {
                            write!(f, "{:} | ", v)?
                        }
                    },
                    None => write!(f, "X")? 
                }
            }
            writeln!(f)?;
            if x < Self::GRID_SIZE - 1
            {
                write!(f, "---------")?;
                writeln!(f)?;
            }
        }

        Ok(())
    }
}