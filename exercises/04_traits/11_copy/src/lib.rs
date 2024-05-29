// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}
use std::ops::Add;
impl Add<Self> for WrappingU32 {
    type Output = WrappingU32;
    fn add(self, rhs: WrappingU32) -> WrappingU32 {
        let num = self.value + rhs.value;
        WrappingU32::new(num)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
