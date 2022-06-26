pub struct CircuitStates {
    pub vec: Vec<u8>,
}

impl CircuitStates {
    pub fn get(&self, id: usize) -> Option<bool> {
        let vec_index = id >> 3;
        let byte_index = id & 0b111;
        let byte = *self.vec.get(vec_index)?;

        Some(((byte >> byte_index) & 1) == 1)
    }

    pub fn set(&mut self, id: usize, state: bool) -> Option<()> {
        let vec_index = id >> 3;
        let byte_index = id & 0b111;
        let byte = self.vec.get_mut(vec_index)?;
        let state = if state { 1 } else { 0 };

        *byte = (*byte & !(1 << byte_index)) | (state << byte_index);

        Some(())
    }
}
