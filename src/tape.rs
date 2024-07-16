pub struct Tape {
    left: Vec<u8>,
    right: Vec<u8>,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn from(s: &str) -> Self {
        let mut right = Vec::new();
        right.extend_from_slice(s.as_bytes());
        Self {
            left: Vec::new(),
            right,
        }
    }

    pub fn get(&self, idx: i32) -> u8 {
        if idx >= 0 {
            let idx = idx as usize;
            self.right.get(idx).copied().unwrap_or(0)
        } else {
            let idx = (!idx) as usize;
            self.left.get(idx).copied().unwrap_or(0)
        }
    }

    pub fn get_reserve(&mut self, idx: i32) -> u8 {
        if idx >= 0 {
            let idx = idx as usize;
            if idx >= self.right.len() {
                self.right.resize(idx + 1, 0);
            }
            self.right[idx]
        } else {
            let idx = (!idx) as usize;
            if idx >= self.left.len() {
                self.left.resize(idx + 1, 0);
            }
            self.left[idx]
        }
    }

    pub fn set(&mut self, idx: i32, val: u8) {
        if idx >= 0 {
            let idx = idx as usize;
            if idx >= self.right.len() {
                self.right.resize(idx + 1, 0);
            }
            self.right[idx] = val;
        } else {
            let idx = (!idx) as usize;
            if idx >= self.left.len() {
                self.left.resize(idx + 1, 0);
            }
            self.left[idx] = val;
        }
    }
}
