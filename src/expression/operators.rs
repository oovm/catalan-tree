pub struct OperatorPermutation<O> {
    order: usize,
    length: usize,
    operators: Vec<O>,
}

impl<O> OperatorPermutation<O> {
    pub fn new(operators: &[ExpressionAction], length: usize) -> Self {
        let mut operators = operators.to_vec();
        operators.dedup();
        Self { order: 0, length, operators }
    }
    pub fn with_order(mut self, order: usize) -> Self {
        self.order = order;
        self
    }
}

impl<O> Iterator for OperatorPermutation<O> {
    type Item = Vec<ExpressionAction>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.order >= self.operators.len().pow(self.length as u32) {
            return None;
        }
        let mut actions = vec![self.operators[0]; self.length];
        let mut pointer = self.order;
        for i in 0..actions.len() {
            actions[i] = self.operators[pointer % self.length];
            pointer /= self.length;
        }
        self.order += 1;
        Some(actions)
    }
}
