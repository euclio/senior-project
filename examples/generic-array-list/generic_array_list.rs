pub struct ArrayList<T> {
    size: usize,
    backing_array: Vec<T>
}

impl<T: Clone> ArrayList<T> {
    pub fn new() -> ArrayList<T> {
        ArrayList { size: 0, backing_array: Vec::with_capacity(10) }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn head(&self) -> Option<T> {
        match self.size > 0 {
            true => Some(self.backing_array[0].clone()),
            false => None
        }
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.size += 1;
        self.backing_array.insert(index, element)
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.size -= 1;
        self.backing_array.remove(index)
    }

    pub fn get(&self, index: usize) -> T {
        self.backing_array[index].clone()
    }
}
