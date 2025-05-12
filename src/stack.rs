pub struct Stack_Pointer {
        pub pointer: usize,
}

pub struct Stack {
        pub stack: [u16; 16],
        pub pointer: Stack_Pointer
}

impl Stack {
        pub fn new() -> Self {
                Stack {
                        stack: [0; 16],
                        pointer: Stack_Pointer { pointer: 0 }
                }
        }

    pub fn push(&mut self, value: u16) {
        if self.pointer.pointer < 16 {
            self.stack[self.pointer.pointer] = value;
            self.pointer.pointer += 1;
        } else {
            panic!("Stack overflow");
        }
    }

        pub fn pop(&mut self) -> u16 {
                if self.pointer.pointer > 0 {
                self.pointer.pointer -= 1;
                let value = self.stack[self.pointer.pointer];
                self.stack[self.pointer.pointer] = 0;
                value
                } else {
                panic!("Stack underflow");
                }
        }
}


