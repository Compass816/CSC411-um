use std::collections::HashMap;
/* Handle everything with memory, registers, program counter(?), ids vec */


pub struct Memory {
    pub data: HashMap<u32, Vec<u32>>,
    pub mem_ids: Vec<u32>,
    pub latest_id: u32,
    pub program_counter: u32
}

impl Memory {
    pub fn new(id: u32, val: Vec<u32>) -> Memory {
        let mut data = HashMap::new();
        data.insert(id, val);

        Memory { 
            data,
            mem_ids: vec![],
            latest_id: 0,
            program_counter: 0
        }
    }

    /// Gets a value at a certain key in the Memory hashmap
    /// # Returns:
    /// * `Option<&Vec<u32>` an option type if the &Vec<u32> has been found at that ID
    /// # Arguments:
    /// * `&self` : Memory hashmap object
    /// * `id: &u32` : reference to an id, which is a key in Memory
    pub fn get(&self, id: &u32) -> Option<&Vec<u32>> {
        self.data.get(id)
    }


    /// Sets a value at a certain key in the Memory hashmap
    /// # Arguments:
    /// * `&self` : Memory hashmap object
    /// * `id: &u32` : reference to an id, which is a key in Memory
    pub fn set(&mut self, id: u32, new_val: Vec<u32>) {
        if self.data.contains_key(&id) {
            self.data.insert(id, new_val);
        }
    }


    /// Adds a new Vec<u32> into Memory at a specified ID
    /// # Arguments:
    /// * `&self` : Memory hashmap object
    /// * `id: &u32` : reference to an id, which is a key in Memory
    pub fn add(&mut self, id: u32, val: Vec<u32>) {
        self.data.insert(id, val);
    }


    // Removes the Vec<u32> in Memory at a specified ID
    /// # Arguments:
    /// * `&self` : Memory hashmap object
    /// * `id: &u32` : reference to an id, which is a key in Memory
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
