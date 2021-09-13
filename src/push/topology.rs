pub struct Topology {}

impl Topology {
    /// Calculates the Euclidean distance between
    /// two index vectors.
    pub fn euclidean(i1: &Vec<usize>, i2: &Vec<usize>) -> f32 {
        0.0
    }

    /// Calculates the index components in each dimension
    /// given the number of elements in each dimension
    pub fn decompose_index(index: &usize, max_idx: &Vec<usize>) -> Vec<usize> {
        let dindex = vec![0; max_idx.len()];
        let i = 9;
        let n = 3;

        for i in 0..(n * n * n * n) {
            let k = i % n;
            let l = (i / n) % n;
            let m = i / (n * n) % n;
            let o = i / (n * n * n);
        }
        dindex
    }
}
