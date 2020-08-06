use std::collections::HashMap;

pub fn test_add(a: u32, b: u32) -> u32 {
    return a + b;
}

// types and bounds
pub type ValType = usize;
pub type PosType = u8;
pub const BoardSize: PosType = 3;

// error types
pub enum ParameterError {
    IncorrectDimensionality,
    PositionOutOfBounds,
    ValueOutOfBounds,
    AlreadyAssigned,
    NotAssigned
}

// board abstraction
struct Board {
    val_cache: HashMap<Vec<PosType>, ValType>, // stores in value efficient format
    pos_vec: Vec<Vec<Vec<PosType>>>, // stores in position efficient format
    val_count: ValType, // number of different values
    dimensions: PosType // number of dimensions
}


// board functions
impl Board {

    // check if given value has won based on last position
    pub fn check_adj(&self, init_pos: &Vec<PosType>) -> Result<bool, ParameterError> {
        let val: Option<&ValType> = self.get_val(init_pos);
        if self.get_val(init_pos).is_none() {
            return Err(ParameterError::NotAssigned)
        }

        // iter through each dimension
        let mut dimension: usize = 0;
        for indices in &self.pos_vec[*val.unwrap() as usize] {
           
            // 3 of same
            if Board::count_pos(indices, init_pos[dimension]) == BoardSize {
                continue
            } 

            // ascending
            let mut checked = [false; BoardSize as usize];            
            for index in indices {
                checked[*index as usize] = true; 
            }
            for index in &checked {
                if !*index {
                    return Ok(false)
                }  
            } 
        }

        // all checks passed
        Ok(true)
    }


    // get value at coordinates
    pub fn get_val(&self, pos: &Vec<PosType>) -> Option<&ValType> {
        self.val_cache.get(pos)
    }


    // set value at coordinates
    pub fn set_val(&mut self, pos: &Vec<PosType>, val: ValType) -> Result<(), ParameterError> {
        if val >= self.val_count {
            return Err(ParameterError::ValueOutOfBounds)
        } 
        match Board::check_pos(&self, pos) {
            Err(e) => Err(e),
            Ok(()) => {
                self.pos_vec[val as usize].push(pos.clone());
                self.val_cache.insert(pos.clone(), val); 
                Ok(())
            }
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
        if self.get_val(pos).is_some() {
            return Err(ParameterError::AlreadyAssigned)
        }
        Ok(())
    }

    // count occurrences of index position in vec
    fn count_pos(index_vec: &Vec<PosType>, index: PosType) -> PosType {
        let mut count: PosType = 0;
        for pos in index_vec {
            if *pos == index { count += 1 } 
        }
        count
    }

}