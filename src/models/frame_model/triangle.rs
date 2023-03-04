use crate::macros::*;

use super::{Edge, Point};
#[derive(Clone, Debug, PartialEq)]
/// Keeps ID's of points, that forms trinagle
pub struct Triangle
{
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle
{
    pub fn new(points_id: &[usize; 3]) -> Self
    {
        Self {
            a: points_id[0],
            b: points_id[1],
            c: points_id[2],
        }
    }
    getter_setter!(a: usize, b: usize, c: usize);

    pub fn abc(&self) -> [usize; 3]
    {
        [self.a, self.b, self.c]
    }

    pub fn from_edges<T>(points: &[Point<T>], edges: &[Edge]) -> Vec<Triangle>
    {
        let mut triangles = Vec::new();

        // Create a hash map of edges for quick lookup
        let mut edge_map = std::collections::HashMap::new();
        for edge in edges {
            let (from, to) = edge.from_to();
            edge_map.insert((from, to), true);
            edge_map.insert((to, from), true);
        }

        // Check every triplet of points to see if it forms a triangle
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                for k in j + 1..points.len() {
                    let mut is_triangle = true;

                    // Check if each pair of points forms an edge in the hash map
                    if edge_map.get(&(i, j)).is_none() {
                        is_triangle = false;
                    }
                    if edge_map.get(&(j, k)).is_none() {
                        is_triangle = false;
                    }
                    if edge_map.get(&(k, i)).is_none() {
                        is_triangle = false;
                    }

                    // If all edges exist, add the triangle to the result
                    if is_triangle {
                        triangles.push(Triangle::new(&[i, j, k]));
                    }
                }
            }
        }

        return triangles;
    }
}
