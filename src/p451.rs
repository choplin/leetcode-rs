struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn frequency_sort(s: String) -> String {
        let mut count = std::collections::HashMap::<char, usize>::new();
        for c in s.chars() {
            let v = count.get(&c).map_or(0, |v| *v);
            count.insert(c, v + 1);
        }
        let mut vec: Vec<(&char, &usize)> = count.iter().collect();
        vec.sort_by_key(|(_, cnt)| **cnt);

        let mut s = String::with_capacity(s.len());
        for (c, cnt) in vec.iter().rev() {
            for _ in 0..**cnt {
                s.push(**c);
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "eert".to_string(),
            Solution::frequency_sort("tree".to_string())
        )
    }
}
