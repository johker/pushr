use std::fmt;

#[derive(Clone)]
pub struct BoolVector {
    values: Vec<bool>,
}

impl BoolVector {
    pub fn new(arg: Vec<bool>) -> Self {
        Self { values: arg }
    }
}

impl fmt::Display for BoolVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self
            .values
            .clone()
            .into_iter()
            .fold(String::new(), |acc, num| {
                acc + &(num as u32).to_string() + ","
            });
        s.pop();
        write!(f, "[{}]", s)
    }
}

impl PartialEq for BoolVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[derive(Clone)]
pub struct IntVector {
    values: Vec<i32>,
}

impl IntVector {
    pub fn new(arg: Vec<i32>) -> Self {
        Self { values: arg }
    }
}

impl fmt::Display for IntVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self
            .values
            .clone()
            .into_iter()
            .fold(String::new(), |acc, num| acc + &num.to_string() + ",");
        s.pop();
        write!(f, "[{}]", s)
    }
}

impl PartialEq for IntVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[derive(Clone)]
pub struct FloatVector {
    values: Vec<f32>,
}

impl FloatVector {
    pub fn new(arg: Vec<f32>) -> Self {
        Self { values: arg }
    }
}

impl fmt::Display for FloatVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self
            .values
            .clone()
            .into_iter()
            .fold(String::new(), |acc, num| acc + &num.to_string() + ",");
        s.pop();
        write!(f, "[{}]", s)
    }
}

impl PartialEq for FloatVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_vector_prints_values() {
        let bv = BoolVector::new(vec![true, false, true]);
        assert_eq!(bv.to_string(), "[1,0,1]");
    }

    #[test]
    fn int_vector_prints_values() {
        let iv = IntVector::new(vec![1, 2, -3]);
        assert_eq!(iv.to_string(), "[1,2,-3]");
    }

    #[test]
    fn float_vector_prints_values() {
        let fv = FloatVector::new(vec![1.2, 3.4, -4.5]);
        assert_eq!(fv.to_string(), "[1.2,3.4,-4.5]");
    }
}
