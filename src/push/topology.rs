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
    /// of dimensions nd
    pub fn decompose_index(index: &usize, nedge: &usize, ndim: &usize) -> Vec<usize> {
        let mut dindex = vec![0; *ndim];
        for i in 0..*ndim {
            dindex[i] = (index / nedge.pow(i as u32)) % nedge;
        }
        dindex
    }

    /// Calculates the indices of the neighbors for a vector of the the total
    /// size ntotal divided in ndim dimensions. A neighbor's euclidean distance to
    /// the given index is smaller than the given radius. The distance is calculated
    /// based on the decomposed index representation.
    pub fn find_neighbors(
        ntotal: &usize,
        ndim: &usize,
        index: &usize,
        radius: &f32,
    ) -> Option<Vec<usize>> {
        if *radius < 0.0 || *ndim < 1 || *index > *ntotal {
            return None;
        }
        let nedge = f32::floor((*ntotal as f32).powf(1.0 / *ndim as f32)) as usize;
        // let overhang = nt - nedge * ndim;
        let dindex = Topology::decompose_index(index, &nedge, ndim);
        let mut neighbors = vec![];
        for i in 0..*ntotal {
            let di = Topology::decompose_index(&i, &nedge, ndim);
            if let Some(dist) = Topology::euclidean_distance(&dindex, &di) {
                if dist <= *radius {
                    neighbors.push(i);
                }
            }
        }
        Some(neighbors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompose_index_without_overhang() {
        assert_eq!(Topology::decompose_index(&14, &6, &2), vec![2, 2]);
        assert_eq!(Topology::decompose_index(&4, &2, &3), vec![0, 0, 1]);
        assert_eq!(Topology::decompose_index(&13, &3, &3), vec![1, 1, 1]);
        assert_eq!(Topology::decompose_index(&26, &3, &3), vec![2, 2, 2]);
        assert_eq!(Topology::decompose_index(&80, &3, &4), vec![2, 2, 2, 2]);
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
    fn find_neighbors_without_overhang() {
        assert_eq!(
            Topology::find_neighbors(&36, &2, &14, &1.0).unwrap(),
            vec![8, 13, 14, 15, 20]
        );
    }
}
