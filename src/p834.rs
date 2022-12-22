struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];
        for v in edges.iter() {
            adj[v[0] as usize].push(v[1] as usize);
            adj[v[1] as usize].push(v[0] as usize);
        }

        let mut ans = vec![0; n];
        let mut count = vec![1; n];

        Self::dfs1(0, n, &mut count, &mut ans, &adj);
        Self::dfs2(0, n, &mut count, &mut ans, &adj);
        ans
    }

    fn dfs1(
        i: usize,
        parent: usize,
        count: &mut Vec<usize>,
        ans: &mut Vec<i32>,
        adj: &Vec<Vec<usize>>,
    ) {
        for &v in &adj[i] {
            if v != parent {
                Self::dfs1(v, i, count, ans, adj);
                count[i] += count[v];
                ans[i] += ans[v] + count[v] as i32;
            }
        }
    }

    fn dfs2(
        i: usize,
        parent: usize,
        count: &mut Vec<usize>,
        ans: &mut Vec<i32>,
        adj: &Vec<Vec<usize>>,
    ) {
        for &v in &adj[i] {
            if v != parent {
                ans[v] = ans[i] - count[v] as i32 + (count.len() - count[v]) as i32;
                Self::dfs2(v, i, count, ans, adj);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![8, 12, 6, 10, 10, 10],
            Solution::sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5],]
            )
        )
    }
}
