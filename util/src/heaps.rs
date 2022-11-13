pub struct HeapsAlgo<T> {
    pub permutations: Vec<Vec<T>>,
}

impl<T: Clone> HeapsAlgo<T> {
    pub fn new(list: &mut [T]) -> Self {
        let mut ha = Self { permutations: Vec::new() };
        ha.generate(list.len(), list);
        ha
    }
    fn generate(&mut self, size: usize, list: &mut [T]) {
        if size == 1 {
            self.permutations.push(Vec::from(list));
        } else {
            self.generate(size - 1, list);
            for i in 0..size - 1 {
                if size % 2 == 0 {
                    list.swap(i, size - 1)
                } else {
                    list.swap(0, size - 1)
                }
                self.generate(size - 1, list);
            }
        }
    }
}
