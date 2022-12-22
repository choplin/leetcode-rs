struct Solution {}

use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut dislikes_per_person: Vec<Vec<usize>> = vec![vec![]; n as usize];

        for d in dislikes.iter() {
            let a = d[0] as usize - 1;
            let b = d[1] as usize - 1;
            dislikes_per_person[a].push(b);
            dislikes_per_person[b].push(a);
        }

        let mut v: Vec<Option<bool>> = vec![None; n as usize];

        for i in 0..n as usize {
            if v[i].is_some() {
                continue;
            }
            let mut vd: VecDeque<(usize, bool)> = VecDeque::new();
            vd.push_back((i, true));
            while let Some(last) = vd.pop_front() {
                if let Some(b) = v[last.0] {
                    if b != last.1 {
                        return false;
                    }
                    continue;
                }
                v[last.0] = Some(last.1);
                for j in dislikes_per_person[last.0].iter() {
                    if v[*j].is_none() {
                        vd.push_back((*j, !last.1));
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4],])
        )
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            false,
            Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3],])
        )
    }

    #[test]
    fn it_works3() {
        assert_eq!(
            false,
            Solution::possible_bipartition(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5],]
            )
        )
    }

    #[test]
    fn it_works4() {
        assert_eq!(
            true,
            Solution::possible_bipartition(
                50,
                vec![
                    vec![21, 47],
                    vec![4, 41],
                    vec![2, 41],
                    vec![36, 42],
                    vec![32, 45],
                    vec![26, 28],
                    vec![32, 44],
                    vec![5, 41],
                    vec![29, 44],
                    vec![10, 46],
                    vec![1, 6],
                    vec![7, 42],
                    vec![46, 49],
                    vec![17, 46],
                    vec![32, 35],
                    vec![11, 48],
                    vec![37, 48],
                    vec![37, 43],
                    vec![8, 41],
                    vec![16, 22],
                    vec![41, 43],
                    vec![11, 27],
                    vec![22, 44],
                    vec![22, 28],
                    vec![18, 37],
                    vec![5, 11],
                    vec![18, 46],
                    vec![22, 48],
                    vec![1, 17],
                    vec![2, 32],
                    vec![21, 37],
                    vec![7, 22],
                    vec![23, 41],
                    vec![30, 39],
                    vec![6, 41],
                    vec![10, 22],
                    vec![36, 41],
                    vec![22, 25],
                    vec![1, 12],
                    vec![2, 11],
                    vec![45, 46],
                    vec![2, 22],
                    vec![1, 38],
                    vec![47, 50],
                    vec![11, 15],
                    vec![2, 37],
                    vec![1, 43],
                    vec![30, 45],
                    vec![4, 32],
                    vec![28, 37],
                    vec![1, 21],
                    vec![23, 37],
                    vec![5, 37],
                    vec![29, 40],
                    vec![6, 42],
                    vec![3, 11],
                    vec![40, 42],
                    vec![26, 49],
                    vec![41, 50],
                    vec![13, 41],
                    vec![20, 47],
                    vec![15, 26],
                    vec![47, 49],
                    vec![5, 30],
                    vec![4, 42],
                    vec![10, 30],
                    vec![6, 29],
                    vec![20, 42],
                    vec![4, 37],
                    vec![28, 42],
                    vec![1, 16],
                    vec![8, 32],
                    vec![16, 29],
                    vec![31, 47],
                    vec![15, 47],
                    vec![1, 5],
                    vec![7, 37],
                    vec![14, 47],
                    vec![30, 48],
                    vec![1, 10],
                    vec![26, 43],
                    vec![15, 46],
                    vec![42, 45],
                    vec![18, 42],
                    vec![25, 42],
                    vec![38, 41],
                    vec![32, 39],
                    vec![6, 30],
                    vec![29, 33],
                    vec![34, 37],
                    vec![26, 38],
                    vec![3, 22],
                    vec![18, 47],
                    vec![42, 48],
                    vec![22, 49],
                    vec![26, 34],
                    vec![22, 36],
                    vec![29, 36],
                    vec![11, 25],
                    vec![41, 44],
                    vec![6, 46],
                    vec![13, 22],
                    vec![11, 16],
                    vec![10, 37],
                    vec![42, 43],
                    vec![12, 32],
                    vec![1, 48],
                    vec![26, 40],
                    vec![22, 50],
                    vec![17, 26],
                    vec![4, 22],
                    vec![11, 14],
                    vec![26, 39],
                    vec![7, 11],
                    vec![23, 26],
                    vec![1, 20],
                    vec![32, 33],
                    vec![30, 33],
                    vec![1, 25],
                    vec![2, 30],
                    vec![2, 46],
                    vec![26, 45],
                    vec![47, 48],
                    vec![5, 29],
                    vec![3, 37],
                    vec![22, 34],
                    vec![20, 22],
                    vec![9, 47],
                    vec![1, 4],
                    vec![36, 46],
                    vec![30, 49],
                    vec![1, 9],
                    vec![3, 26],
                    vec![25, 41],
                    vec![14, 29],
                    vec![1, 35],
                    vec![23, 42],
                    vec![21, 32],
                    vec![24, 46],
                    vec![3, 32],
                    vec![9, 42],
                    vec![33, 37],
                    vec![7, 30],
                    vec![29, 45],
                    vec![27, 30],
                    vec![1, 7],
                    vec![33, 42],
                    vec![17, 47],
                    vec![12, 47],
                    vec![19, 41],
                    vec![3, 42],
                    vec![24, 26],
                    vec![20, 29],
                    vec![11, 23],
                    vec![22, 40],
                    vec![9, 37],
                    vec![31, 32],
                    vec![23, 46],
                    vec![11, 38],
                    vec![27, 29],
                    vec![17, 37],
                    vec![23, 30],
                    vec![14, 42],
                    vec![28, 30],
                    vec![29, 31],
                    vec![1, 8],
                    vec![1, 36],
                    vec![42, 50],
                    vec![21, 41],
                    vec![11, 18],
                    vec![39, 41],
                    vec![32, 34],
                    vec![6, 37],
                    vec![30, 38],
                    vec![21, 46],
                    vec![16, 37],
                    vec![22, 24],
                    vec![17, 32],
                    vec![23, 29],
                    vec![3, 30],
                    vec![8, 30],
                    vec![41, 48],
                    vec![1, 39],
                    vec![8, 47],
                    vec![30, 44],
                    vec![9, 46],
                    vec![22, 45],
                    vec![7, 26],
                    vec![35, 42],
                    vec![1, 27],
                    vec![17, 30],
                    vec![20, 46],
                    vec![18, 29],
                    vec![3, 29],
                    vec![4, 30],
                    vec![3, 46]
                ]
            )
        )
    }
}
