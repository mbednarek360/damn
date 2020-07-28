use std::collections::HashMap;

pub fn test_add(a: u32, b: u32) -> u32 {
    return a + b;
}

// types and bounds
pub type ValType = u8;
pub type PosType = u8;
pub const BoardSize: PosType = 3;

// error types
pub enum ParameterError {
    IncorrectDimensionality,
    PositionOutOfBounds,
    ValueOutOfBounds
}

// board abstraction
struct Board {
    data: HashMap<Vec<PosType>, ValType>, // coordinate value pairs
    val_count: ValType, // number of different values
    dimensions: PosType // number of dimensions
}

// board functions
impl Board {

    // check if given value has won based on last position
    pub fn check_adj(&self, val: ValType, pos: Vec<PosType>) -> Result<bool, ParameterError> {
        if val >= self.val_count {
            return Err(ParameterError::ValueOutOfBounds)
        }
        Ok(false) 
    }


    // get value at coordinates
    pub fn get_val(&self, pos: Vec<PosType>) -> Option<&ValType> {
        self.data.get(&pos)
    }


    // set value at coordinates
    pub fn set_val(&mut self, pos: Vec<PosType>, val: ValType) -> Result<(), ParameterError> {
        if val >= self.val_count {
            return Err(ParameterError::ValueOutOfBounds)
        } 
        match Board::check_pos(&self, &pos) {
            Err(e) => Err(e),
            Ok(()) => {self.data.insert(pos, val); Ok(())}
        }
    }










    // ensure valid position vector
    fn check_pos(&self, pos: &Vec<PosType>) -> Result<(), ParameterError> {
        if (&pos).len() as PosType != self.dimensions {
            return Err(ParameterError::IncorrectDimensionality)
        }
        for coord in pos {
            if *coord >= BoardSize {
                return Err(ParameterError::PositionOutOfBounds)
            }
        } 
        Ok(())
    }
}

  



// first length = number of keys
// second length = dynamic
// third length = number of dimensions



