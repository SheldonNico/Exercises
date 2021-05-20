use std::collections::HashSet;

/// return true if p0-p1 to p0-p1 is clockwise
/// > 0: clockwise
/// < 0: counterclockwise
fn jarvis_orientation((p0x, p0y): &(i32, i32), (p1x, p1y): &(i32, i32), (p2x, p2y): &(i32, i32)) -> i32 {
    // ratio1 > ratio2
    (p1y - p0y) * (p2x - p0x) - (p2y - p0y) * (p1x - p0x)
}

/// TODO: not right
pub fn jarvis_solve(points: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    if points.len() <= 3 { return points.clone().into_iter().collect(); };

    let select: HashSet<usize> = (0..points.len()).into_iter().collect();

    let mut left_most = 0;
    for &idx in select.iter() {
        let p = points[left_most];
        if p.0 > points[idx].0 {
            left_most = idx;
        }
    }

    let mut curr = left_most;
    let mut walked = HashSet::new();
    walked.insert(curr);
    // println!("{:?}", points[curr]);

    loop {
        let mut next = 0;
        for idx in 0..points.len() {
            if next == curr {
                next = idx;
            } else if idx == curr {
            } else {
                // println!(">>> {:?} {:?} {:?}", &points[curr], &points[next], &points[idx]);
                if jarvis_orientation(&points[curr], &points[next], &points[idx]) > 0 {
                    // println!("\t -> {:?}", points[next]);
                    println!("{} {} {}", curr, next, idx);
                    next = idx;
                }
            }
        }

        // save points inside path
        if curr != next {
            for idx in 0..points.len() {
                if idx == curr || idx == next { continue; }
                if jarvis_orientation(&points[curr], &points[next], &points[idx]) == 0 {
                    walked.insert(idx);
                }
            }
        }

        // println!("---- {:?} {}", walked, next);

        if walked.contains(&next) { break; }
        walked.insert(next);
        curr = next;
    }

    walked.into_iter().map(|i| points[i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garvis_sample() {
        fn test<const N: usize>(ori: Vec<[i32; N]>, expect: Vec<[i32; N]>) {
            assert_eq!(jarvis_solve(&ori.into_iter().map(|v| (v[0], v[1])).collect()), expect.into_iter().map(|v| (v[0], v[1])).collect());
        }

        test(
            vec![[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]],
            vec![[1,1],[2,0],[3,3],[2,4],[4,2]]
        );
        // assert_eq!(jarvis_solve(&vec![(1,1),(2,2),(2,0),(2,4),(3,3),(4,2)]), vec![(1, 1), (2, 0), (4, 2), (3, 3), (2, 4)]);
        // test(
        //     vec![[3,0],[4,0],[5,0],[6,1],[7,2],[7,3],[7,4],[6,5],[5,5],[4,5],[3,5],[2,5],[1,4],[1,3],[1,2],[2,1],[4,2],[0,3]],
        //     vec![[7,4],[5,0],[7,3],[2,1],[5,5],[4,5],[3,5],[7,2],[1,2],[1,4],[4,0],[2,5],[6,1],[6,5],[0,3],[3,0]]
        // );
    }

}
