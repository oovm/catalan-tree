pub struct OperatorPermutation<O>
where
    O: Clone,
{
    order: usize,
    length: usize,
    operators: Vec<O>,
}

impl<O> OperatorPermutation<O>
where
    O: Clone,
{
    pub fn new(operators: &[O], length: usize) -> Self
    where
        O: PartialEq,
    {
        let mut operators = operators.to_vec();
        operators.dedup();
        Self { order: 0, length, operators }
    }
    pub fn with_order(mut self, order: usize) -> Self {
        self.order = order;
        self
    }
}

impl<O> Iterator for OperatorPermutation<O>
where
    O: Clone,
{
    type Item = Vec<O>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.order >= self.operators.len().pow(self.length as u32) {
            return None;
        }
        let mut digits = vec![self.operators[0].clone(); self.length];
        let mut rest = self.order;
        // convert ot base self.length
        for i in 0..digits.len() {
            digits[i] = self.operators[rest % self.operators.len()].clone();
            rest /= self.operators.len();
        }
        self.order += 1;
        Some(digits)
    }
}
