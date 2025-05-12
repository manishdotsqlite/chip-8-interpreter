pub struct Memory {
        space: [u8; 4096]
}

impl Memory {

    pub fn new() -> Self {
        Memory {
            space: [0; 4096]
        }
    }
    
    pub fn inject_instructions(&mut self, data: &[u8], location: usize) {
        let last_index: usize = data.len() + location;

        if last_index > 4095 || location < 512  {
            return;
        }

        for i in location..=last_index {
            self.space[i] = data[i - location];
        }
    }

    pub fn fetch_instruction(&self, location: usize) -> u16 {
        let byte1 = self.space[location];
        let byte2 = self.space[location+1];

        let instruction_bytes: u16 = ((byte1 as u16) << 8) | (byte2 as u16);
        instruction_bytes
    }

    pub fn fetch_data(&self, location: usize) -> u8 {
        self.space[location]
    }

}