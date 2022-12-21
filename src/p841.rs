struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut count = 0;
        let mut stack: Vec<usize> = Vec::new();
        let mut seen: Vec<bool> = std::iter::repeat(false).take(rooms.len()).collect();

        count += 1;
        seen[0] = true;
        rooms[0].iter().for_each(|n| {
            count += 1;
            seen[*n as usize] = true;
            stack.push(*n as usize);
        });

        while let Some(key) = stack.pop() {
            rooms[key].iter().for_each(|n| {
                if !seen[*n as usize] {
                    count += 1;
                    seen[*n as usize] = true;
                    stack.push(*n as usize);
                }
            });
        }
        count == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![],])
        );
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            false,
            Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0],])
        );
    }

    #[test]
    fn it_works3() {
        assert_eq!(
            true,
            Solution::can_visit_all_rooms(vec![vec![1, 2], vec![2, 1], vec![1],])
        );
    }
}
