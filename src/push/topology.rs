use crate::push::vector::IntVector;

pub struct Topology {}

impl Topology {
    /// Calculates the Euclidean distance between two index vectors.
    /// Returns None if the vector size dont match.
    pub fn euclidean_distance(i1: &Vec<usize>, i2: &Vec<usize>) -> Option<f32> {
        if i1.len() != i2.len() {
            None
        } else {
            let mut dist = 0.0;
            for i in 0..i1.len() {
                dist += (i1[i] as f32 - i2[i] as f32).powf(2.0);
            }
            Some(f32::sqrt(dist))
        }
    }

    /// Calculates the index components in each dimension
    /// given the edgex length of the hypercube nedge and the number
    /// of dimensions ndim
    pub fn decompose_index(index: &usize, nedge: &usize, ndim: &usize) -> Option<Vec<usize>> {
        let mut dindex = vec![0; *ndim];
        for i in 0..*ndim {
            if let Some(cp) = (*nedge).checked_pow(i as u32) {
                dindex[i] = index / cp % *nedge;
            } else {
                return None;
            }
        }
        Some(dindex)
    }

    /// Calculates the indices of the neighbors for a vector of the the total
    /// size ntotal divided in ndim dimensions. A neighbor's euclidean distance to
    /// the given index is smaller equal to the given radius. The distance is calculated
    /// based on the decomposed index representation. The edge length is computed for
    /// smallest hypercube that contains all indices, e.g 37 elements with 2 dimensions
    /// leads to an edge length 7, where elements 38-42 are ignored.
    ///
    pub fn find_neighbors(
        ntotal: &usize,
        ndim: &usize,
        index: &usize,
        radius: &f32,
    ) -> Option<IntVector> {
        if *radius < 0.0 || *ndim < 1 || *ntotal < 1 || *index > *ntotal {
            return None;
        }
        let nedge = f32::ceil((*ntotal as f32).powf(1.0 / *ndim as f32)) as usize;
        if let Some(dindex) = Topology::decompose_index(index, &nedge, ndim) {
            let mut neighbors = vec![];
            for i in 0..*ntotal {
                if let Some(di) = Topology::decompose_index(&i, &nedge, ndim) {
                    if let Some(dist) = Topology::euclidean_distance(&dindex, &di) {
                        if dist <= *radius {
                            neighbors.push(i as i32);
                        }
                    }
                }
            }
            return Some(IntVector::new(neighbors));
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompose_index_without_overhang() {
        assert_eq!(Topology::decompose_index(&14, &6, &2).unwrap(), vec![2, 2]);
        assert_eq!(
            Topology::decompose_index(&4, &2, &3).unwrap(),
            vec![0, 0, 1]
        );
        assert_eq!(
            Topology::decompose_index(&13, &3, &3).unwrap(),
            vec![1, 1, 1]
        );
        assert_eq!(
            Topology::decompose_index(&26, &3, &3).unwrap(),
            vec![2, 2, 2]
        );
        assert_eq!(
            Topology::decompose_index(&80, &3, &4).unwrap(),
            vec![2, 2, 2, 2]
        );
    }

    #[test]
    fn euclidean_distance_calculated_when_vector_lengths_match() {
        assert_eq!(
            Topology::euclidean_distance(&vec![0, 0], &vec![1, 1, 1]),
            None
        );
        assert_eq!(
            Topology::euclidean_distance(&vec![0, 0], &vec![1, 1]),
            Some(f32::sqrt(2.0))
        );
        assert_eq!(
            Topology::euclidean_distance(&vec![1, 2, 4], &vec![1, 2, 4]),
            Some(0.0)
        );
        assert_eq!(
            Topology::euclidean_distance(&vec![2, 4], &vec![3, 6]),
            Topology::euclidean_distance(&vec![3, 6], &vec![2, 4])
        );
    }

    #[test]
    fn find_neighbors_without_envelope() {
        assert_eq!(
            Topology::find_neighbors(&36, &2, &14, &1.0).unwrap(),
            IntVector::new(vec![8, 13, 14, 15, 20])
        );
        assert_eq!(
            Topology::find_neighbors(&36, &1, &14, &1.0).unwrap(),
            IntVector::new(vec![13, 14, 15])
        );
        assert_eq!(
            Topology::find_neighbors(&27, &3, &13, &f32::sqrt(3.0)).unwrap(),
            IntVector::new(vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26
            ])
        );
    }
    #[test]
    fn find_neighbors_empty() {
        assert_eq!(Topology::find_neighbors(&0, &2, &0, &1.0), None);
    }

    #[test]
    fn find_neighbors_with_envelope() {
        assert_eq!(
            Topology::find_neighbors(&38, &2, &31, &f32::sqrt(2.0)).unwrap(),
            IntVector::new(vec![23, 24, 25, 30, 31, 32, 37])
        );
    }
}
