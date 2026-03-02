struct Solution;

impl Solution {
    // https://leetcode.cn/problems/find-if-path-exists-in-graph/
    #[allow(dead_code)]
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut uf = UnionFind::new(n);
        for edge in edges {
            uf.union(edge[0], edge[1]);
        }
        uf.find(source) == uf.find(destination)
    }
}

struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        UnionFind { parent: (0..n).collect::<Vec<_>>() }
    }

    fn find(&mut self, x: i32) -> i32 {
        let next = self.parent[x as usize];
        if x != next {
            self.parent[x as usize] = self.find(next);
        }
        return self.parent[x as usize];
    }

    fn union(&mut self, x: i32, y: i32) {
        let (x, y) = (self.find(x) as usize, self.find(y) as usize);
        self.parent[x] = self.parent[y];
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let n = 3;
        let edges = vec![vec![0,1],vec![1,2],vec![2,0]];
        let source = 0;
        let destination = 2;
        let ans = Solution::valid_path(n, edges, source, destination);
        assert!(ans);
    }

    #[test]
    fn test1() {
        let n = 6;
        let edges = vec![vec![0,1],vec![0,2],vec![3,5],vec![5,4],vec![4,3]];
        let source = 0;
        let destination = 5;
        let ans = Solution::valid_path(n, edges, source, destination);
        assert!(!ans);
    }
}