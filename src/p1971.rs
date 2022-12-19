struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            return true;
        }

        let mut seen: Vec<bool> = std::iter::repeat(false).take(edges.len()).collect();

        let mut queue: Vec<i32> = vec![source];

        while let Some(target) = queue.pop() {
            for (i, e) in edges.iter().enumerate() {
                if seen[i] {
                    continue;
                }

                let rest = if e[0] == target {
                    e[1]
                } else if e[1] == target {
                    e[0]
                } else {
                    continue;
                };

                seen[i] = true;
                if rest == destination {
                    return true;
                }
                queue.push(rest);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0],], 0, 2)
        )
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            false,
            Solution::valid_path(
                6,
                vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3],],
                0,
                5
            )
        )
    }
}
