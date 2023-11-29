use std::collections::HashMap;
/* Handle everything with memory, registers, program counter(?), ids vec */


pub struct Memory {
    data: HashMap<u32, Vec<u32>>,
    mem_ids: Vec<u32>
}

impl Memory {
    pub fn new(id: u32, val: Vec<u32>) -> Memory {
        let mut data = HashMap::new();
        data.insert(id, val);

        Memory { 
            data,
            mem_ids: vec![]
        }
    }

    pub fn get(&self, id: &u32) -> Option<&Vec<u32>> {
        self.data.get(id)
    }

    pub fn add(&mut self, id: u32, val: Vec<u32>) {
        self.data.insert(id, val);
    }

    pub fn remove(&mut self, id: &u32) {
        self.data.remove(id);
    }
}


pub struct Registers {
    pub data: [u32; 8],
}

impl Default for Registers {
    fn default() -> Self {
        Registers { data: [0; 8] }
    }
}
