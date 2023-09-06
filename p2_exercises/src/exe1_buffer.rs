struct Buffer<T>{
    content:Vec<T>,
}

impl<T> Buffer<T>{
    pub fn sum(&self) -> T
    where T:std::ops::Add<Output = T> + Copy + Default
    {
        let mut sum = T::default();
        for i in &self.content{
            sum = sum + *i;
        }
        sum
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let buffer = Buffer{content:vec![1,2,3,4,5]};
        assert_eq!(buffer.sum(),15);
    }
}
