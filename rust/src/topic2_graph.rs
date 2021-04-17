use std::collections::{HashMap, HashSet};

fn p0997_find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if n == 1 && trust.len() == 0 { return 1; }
    assert!(n > 0);
    let mut freqs: HashMap<i32, usize> = HashMap::new();
    for tr in trust.iter() {
        assert_eq!(tr.len(), 2);
        *freqs.entry(tr[1]).or_default() += 1;
    }

    let mut out = -1;
    for (k, freq) in freqs.iter() {
        if *freq == (n - 1) as usize {
            if out == -1 {
                out = *k as _;
            } else {
                out = -1;
                break;
            }
        }
    }

    if out > 0 {
        for tr in trust.iter() {
            if tr[0] == out {
                out = -1;
            }
        }
    }

    out
}

fn p0797_go(graph: &Vec<Vec<i32>>, path: &mut HashMap<(i32, i32), Vec<Vec<i32>>>, start: i32, end: i32) {
    if path.contains_key(&(start, end)) { return; }

    path.insert((start, end), vec![]);
    for n in graph[start as usize].iter() {
        if *n == end {
            path.get_mut(&(start, end)).unwrap().push(vec![start, end]);
            break;
        }
    }

    for mid in graph[start as usize].iter() {
        p0797_go(graph, path, *mid, end);

        let mut midpath = path.get(&(*mid, end)).unwrap().clone();
        if midpath.len() != 0 {
            for trace in midpath.iter_mut() {
                trace.insert(0, start);
            }
            path.get_mut(&(start, end)).unwrap().append(&mut midpath);
        }
    }
}

fn p0797_go2(graph: &Vec<Vec<i32>>, res: &mut Vec<Vec<i32>>, trace: &mut Vec<i32>, curr: i32) {
    trace.push(curr);
    if curr as usize == graph.len() - 1 {
        res.push(trace.clone());
    } else {
        for num in graph[curr as usize].iter() {
            p0797_go2(graph, res, trace, *num);
        }
    }
    trace.pop();
}

pub fn p0797_all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // let mut path = Default::default();
    // p0797_go(&graph, &mut path, 0, (graph.len() - 1) as i32);
    // path.get_mut(&(0, (graph.len() - 1) as i32)).unwrap().clone()

    let mut res = vec![]; let mut trace = vec![];
    p0797_go2(&graph, &mut res, &mut trace, 0);
    res
}

pub fn p1557_find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    // let mut out: HashSet<i32> = (0..n).into_iter().collect();
    // let mut remove: HashSet<i32> = Default::default();
    // for edge in edges.into_iter() {
    //     assert_eq!(edge.len(), 2);
    //     if remove.contains(&edge[0]) || out.remove(&edge[1]) {
    //         remove.insert(edge[1]);
    //         out.remove(&edge[1]);
    //     }
    // }

    // out.into_iter().collect()

    let mut reachable: Vec<_> = vec![true; n as usize];
    for edge in edges.into_iter() {
        assert_eq!(edge.len(), 2);
        reachable[edge[1] as usize] = false;
    }
    reachable.into_iter().enumerate().flat_map(|(idx, f)| if f { Some(idx as i32) } else { None }).collect()
}

fn p1387_get_power(num: i32, cached: &mut HashMap<i32, usize>) -> usize {
    if cached.contains_key(&num) { return *cached.get(&num).unwrap(); }
    if num == 1 { return 0; }

    let out = if num % 2 == 0 {
        p1387_get_power(num / 2, cached)
    } else {
        p1387_get_power(num * 3 + 1, cached)
    };

    cached.insert(num, out + 1);
    out + 1
}

fn p1387_get_power2(mut num: i32) -> usize {
    let mut step = 0;
    while num > 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = num * 3 + 1;
        }
        step += 1;
    }
    step
}

pub fn p1387_get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let ans: Vec<i32> = (lo..hi+1).into_iter().collect();
    let mut power = vec![];
    // let mut cached: HashMap<i32, usize> = HashMap::new();

    for num in ans.iter() {
        // power.push(p1387_get_power(*num, &mut cached));
        power.push(p1387_get_power2(*num));
    }

    let mut out: Vec<_> = power.into_iter().zip(ans).collect();
    out.sort();
    // eprintln!("{:?}", out);
    out[k as usize - 1].1
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum P0959_Dir {
    Left,
    Right,
    Down,
    Up
}

impl P0959_Dir {
    pub fn next(&self, idx: usize, idy: usize, idz: usize) -> (isize, isize, isize) {
        match *self {
            P0959_Dir::Left  => (idx as isize, idy as isize - 1, 2),
            P0959_Dir::Right => (idx as isize, idy as isize + 1, 0),
            P0959_Dir::Up    => (idx as isize - 1, idy as isize, 3),
            P0959_Dir::Down  => (idx as isize + 1, idy as isize, 1),
        }
    }
}

pub fn p0959_go((idx, idy, idz): (usize, usize, usize), regions: &mut Vec<Vec<Vec<bool>>>, grid: &Vec<Vec<char>>, left: &mut Vec<(usize, usize, usize)>, count: &mut usize) {
    let dirs = match grid[idx][idy] {
        ' ' => {
            for z in 0..4 {
                regions[idx][idy][z] = false;
            }
                // 4 directions
            vec![P0959_Dir::Down, P0959_Dir::Up, P0959_Dir::Left, P0959_Dir::Right]
        },
        '/' => {
            if idz == 0 || idz == 1 {
                // go left or up
                regions[idx][idy][0] = false;
                regions[idx][idy][1] = false;
                vec![P0959_Dir::Up, P0959_Dir::Left]
            } else {
                // go right or down
                regions[idx][idy][2] = false;
                regions[idx][idy][3] = false;
                vec![P0959_Dir::Down, P0959_Dir::Right]
            }
        },
        '\\' => {
            if idz == 0 || idz == 3 {
                regions[idx][idy][0] = false;
                regions[idx][idy][3] = false;
                // go left or down
                vec![P0959_Dir::Down, P0959_Dir::Left]
            } else {
                regions[idx][idy][1] = false;
                regions[idx][idy][2] = false;
                // go right or up
                vec![P0959_Dir::Up, P0959_Dir::Right]
            }
        },
        _ => unreachable!()
    };

    for dir in dirs.into_iter() {
        let (ndx, ndy, ndz) = dir.next(idx, idy, idz);
        if ndx >= 0 && (ndx as usize) < grid.len() && ndy >= 0 && (ndy as usize) < grid.len() && ndz >= 0 && ndz < 4 {
            if regions[ndx as usize][ndy as usize][ndz as usize] {
                left.push((ndx as usize, ndy as usize, ndz as usize));
            }
        }
    }

    if let Some(next) = left.pop() {
        p0959_go(next, regions, grid, left, count);
    } else {
        *count += 1;
        left.clear();

        for ndx in 0..grid.len() {
            for ndy in 0..grid.len() {
                for ndz in 0..3 {
                    if regions[ndx][ndy][ndz] {
                        p0959_go((ndx, ndy, ndz), regions, grid, left, count);
                        return;
                    }
                }
            }
        }
    }
}

pub fn p0959_regions_by_slashes(grid: Vec<String>) -> i32 {
    let n = grid.len();
    let mut regions = vec![vec![vec![true; 4]; n]; n];
    let grid: Vec<Vec<char>> = grid.into_iter().map(|line| line.chars().collect()).collect();

    let mut left = vec![];
    let mut count = 0;

    p0959_go((0, 0, 0), &mut regions, &grid, &mut left, &mut count);

    count as _
}

fn p0841_dfs(curr: usize, rooms: &Vec<Vec<i32>>, walked: &mut HashSet<usize>) {
    walked.insert(curr);
    for next in rooms[curr].iter() {
        if !walked.contains(&(*next as usize)) {
            p0841_dfs(*next as usize, rooms, walked)
        }
    }
}

pub fn p0841_can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut walked = HashSet::new();
    p0841_dfs(0, &rooms, &mut walked);

    walked.len() == rooms.len()
}

fn p0684_dfs(curr: i32, trace: &mut Vec<i32>, walked_points: &mut HashSet<i32>, walked_edge: &mut HashSet<usize>, edges: &Vec<Vec<i32>>) -> bool {
    trace.push(curr);
    if walked_points.contains(&curr) { return true; }
    walked_points.insert(curr);

    for (eid, edge) in edges.iter().enumerate() {
        if walked_edge.contains(&eid) { continue; }

        let mut next = -1;
        if edge[0] == curr {
            next = edge[1];
        } else if edge[1] == curr {
            next = edge[0];
        }

        if next > 0 {
            walked_edge.insert(eid);
            if p0684_dfs(next, trace, walked_points, walked_edge, edges) {
                return true;
            } else {
                trace.pop();
            }
        }
    }
    false
}

pub fn p0684_find(parents: &mut Vec<usize>, ranks: &Vec<usize>, node: usize) -> usize {
    if node != parents[node] {
        parents[node] = p0684_find(parents, ranks, parents[node]);
    }

    parents[node]
}

pub fn p0684_union(parents: &mut Vec<usize>, ranks: &mut Vec<usize>, n1: usize, n2: usize) -> bool {
    let r1 = p0684_find(parents, ranks, n1);
    let r2 = p0684_find(parents, ranks, n2);
    if r1 == r2 { return false; }

    if ranks[r1] > ranks[r2] {
        parents[r2] = r1;
    } else {
        if ranks[r1] == ranks[r2] { ranks[r2] += 1; }
        parents[r1] = r2;
    }
    return true;
}

pub fn p0684_find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    // let mut trace = vec![];
    // let mut walked_points = HashSet::new();
    // let mut walked_edge = HashSet::new();

    // p0684_dfs(1, &mut trace, &mut walked_points, &mut walked_edge, & edges);
    // let last = trace.last().unwrap(); let idx = trace.iter().position(|&r| r == *last).unwrap();
    // let trace: Vec<i32> = trace[idx..].into_iter().map(|s| *s).collect();

    // let mut res = vec![];
    // for (s, e) in trace.iter().cycle().zip(trace.iter().cycle().skip(1).take(trace.len() - 1)) {
    //     for (eid, edge) in edges.iter().enumerate() {
    //         if (edge[0] == *s && edge[1] == *e) || (edge[1] == *s && edge[0] == *e) {
    //             res.push((eid, edge));
    //             break;
    //         }
    //     }
    // }
    // res.sort();

    // res.last().unwrap().1.clone()




    // let mut parent: Vec<usize>= (0..(edges.len()+1)).into_iter().collect();
    // let mut res = vec![];
    // for edge in edges.iter() {
    //     let mut n0 = edge[0] as usize; let mut n1 = edge[1] as usize;
    //     while parent[n0 as usize] != n0 { n0 = parent[n0] }
    //     while parent[n1 as usize] != n1 { n1 = parent[n1] }
    //     if n0 == n1 {
    //         res = edge.clone();
    //     } else {
    //         parent[n0] = n1;
    //     }
    // }

    // res


    let mut res = vec![];
    let mut parents: Vec<_> = (0..(edges.len()+1)).into_iter().collect();
    let mut ranks: Vec<_> = vec![0; edges.len()+1];
    for edge in edges.iter() {
        let n0 = edge[0]; let n1 = edge[1];
        if !p0684_union(&mut parents, &mut ranks, n0 as usize, n1 as usize) {
            res = edge.clone();
        }
    }

    return res;
}

pub fn p1267_count_servers(grid: Vec<Vec<i32>>) -> i32 {
    // let n = grid.len();
    // if n == 1 { return 0; }
    // let m = grid[0].len();
    // let mut comm = vec![vec![0; m]; n];

    // for i in 0..n {
    //     if grid[i].iter().sum::<i32>() > 1 {
    //         for j in 0..m {
    //             comm[i][j] = grid[i][j];
    //         }
    //     }
    // }
    // for j in 0..m {
    //     let mut sum = 0;
    //     for i in 0..n { sum += grid[i][j]; }
    //     if sum > 1 {
    //         for i in 0..n {
    //             comm[i][j] = grid[i][j];
    //         }
    //     }
    // }

    // let mut sum = 0;
    // for i in 0..n {
    //     for j in 0..m {
    //         sum += comm[i][j];
    //     }
    // }

    // sum

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let n = grid.len();
    if n == 1 { return 0; }
    let m = grid[0].len();
    let mut row_count = vec![0; n];
    let mut col_count = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            row_count[i] += grid[i][j];
            col_count[j] += grid[i][j];
        }
    }

    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] > 0 && row_count[i] + col_count[j] > 2 {
                count += 1;
            }
        }
    }

    count
}

fn p0399_get(left: &str, right: &str, answers: &mut HashMap<(String, String), Option<f64>>, equations: &Vec<Vec<String>>, values: &Vec<f64>, processing: &mut HashSet<(String, String)>) -> Option<f64> {
    let key = (left.to_string(), right.to_string());
    if answers.contains_key(&key) { return answers.get(&key).unwrap().clone(); }
    if processing.contains(&(left.to_string(), right.to_string())) { return None; }
    if processing.contains(&(right.to_string(), left.to_string())) { return None; }
    processing.insert((left.to_string(), right.to_string()));

    let mut ans = None;
    let mut ans_rev = None;
    for (equal, val) in equations.iter().zip(values.iter()) {
        let e_left = &equal[0]; let e_right = &equal[1];
        if e_left == left && e_right == right {
            ans = Some(*val);
            ans_rev = Some(1. / *val);
            break;
        } else if e_left == right && e_right == left {
            ans = Some(1. / val);
            ans_rev = Some(*val);
            break;
        } else {
            if e_left == left {
                if let Some(other) = p0399_get(e_right, right, answers, equations, values, processing) {
                    ans = Some(val * other);
                    ans_rev = Some(1. / (val * other));
                    break;
                }
            } else if e_right == left {
                if let Some(other) = p0399_get(e_left, right, answers, equations, values, processing) {
                    ans = Some((1. / val) * other);
                    ans_rev = Some(val / other);
                    break;
                }
            } else if e_left == right {
                if let Some(other) = p0399_get(left, e_right, answers, equations, values, processing) {
                    ans = Some((1. / val) * other);
                    ans_rev = Some(val / other);
                    break;
                }
            } else if e_right == right {
                if let Some(other) = p0399_get(left, e_left, answers, equations, values, processing) {
                    ans = Some(val * other);
                    ans_rev = Some(1. / (val * other));
                    break;
                }
            }
        }
    }

    answers.insert(key, ans);
    answers.insert((right.to_string(), left.to_string()), ans_rev);
    processing.remove(&(left.to_string(), right.to_string()));

    ans
}

pub fn p0399_calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut answers: HashMap<(String, String), Option<f64>> = HashMap::new();
    let mut out = Vec::with_capacity(queries.len());
    let mut processing = HashSet::new();
    for query in queries.iter() {
        out.push(p0399_get(&query[0], &query[1], &mut answers, &equations, &values, &mut processing).unwrap_or(-1.));
    }

    out
}

pub fn p1615_maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut dir_nodes: HashMap<i32, HashSet<i32>> = (0..n).into_iter().map(|k| (k, HashSet::new())).collect();
    for road in roads.iter() {
        let n1 = road[0]; let n2 = road[1];
        dir_nodes.get_mut(&n1).unwrap().insert(n2);
        dir_nodes.get_mut(&n2).unwrap().insert(n1);
    }

    let mut rank = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            let mut r = dir_nodes.get(&i).unwrap().len() + dir_nodes.get(&j).unwrap().len();
            if dir_nodes.get(&i).unwrap().contains(&j) { r -= 1; }
            rank = rank.max(r);
        }
    }

    rank as i32
}

fn p1631_dfs_with_dp(
    (sx, sy): (usize, usize), heights: &Vec<Vec<i32>>, trace: &mut Vec<(usize, usize)>, cost: i32,
    answers: &mut HashMap<(usize, usize), Option<i32>>
) -> Option<i32> {
    let m = heights.len(); let n = heights[0].len(); trace.push((sx, sy));
    if (sx, sy) == (m-1, n-1) { return Some(cost); }
    if let Some(v) = answers.get(&(sx, sy)) { return v.clone(); }

    let mut effs = vec![];

    if sx > 0 {
        let (nx, ny) = (sx-1, sy);
        let effort = (heights[sx][sy] - heights[nx][ny]).abs();
        let trace_pos = trace.len();
        if trace.iter().position(|s| s == &(nx, ny)).is_none() {
            if let Some(e) = p1631_dfs_with_dp((nx, ny), heights, trace, cost.max(effort), answers) {
                effs.push(e);
            }
            let _r = trace.split_off(trace_pos);
        }
    }

    if sy > 0 {
        let (nx, ny) = (sx, sy-1);
        let effort = (heights[sx][sy] - heights[nx][ny]).abs();
        let trace_pos = trace.len();
        if trace.iter().position(|s| s == &(nx, ny)).is_none() {
            if let Some(e) = p1631_dfs_with_dp((nx, ny), heights, trace, cost.max(effort), answers) {
                effs.push(e);
            }
            let _r = trace.split_off(trace_pos);
        }
    }

    if sx+1 < m {
        let (nx, ny) = (sx+1, sy);
        let effort = (heights[sx][sy] - heights[nx][ny]).abs();
        let trace_pos = trace.len();
        if trace.iter().position(|s| s == &(nx, ny)).is_none() {
            if let Some(e) = p1631_dfs_with_dp((nx, ny), heights, trace, cost.max(effort), answers) {
                effs.push(e);
            }
            let _r = trace.split_off(trace_pos);
        }
    }

    if sy+1 < n {
        let (nx, ny) = (sx, sy+1);
        let effort = (heights[sx][sy] - heights[nx][ny]).abs();
        let trace_pos = trace.len();
        if trace.iter().position(|s| s == &(nx, ny)).is_none() {
            if let Some(e) = p1631_dfs_with_dp((nx, ny), heights, trace, cost.max(effort), answers) {
                effs.push(e);
            }
            let _r = trace.split_off(trace_pos);
        }
    }

    eprintln!("{:?} {:?} {:?} {:?}", (sx, sy), answers, effs, trace);
    trace.pop();
    if let Some(v) = effs.into_iter().min() {
        answers.insert((sx, sy), Some(v));
        Some(v)
    } else {
        None
    }
}

// TODO
pub fn p1631_minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let m = heights.len(); let n = heights[0].len(); let mut trace = vec![]; let mut answers = HashMap::new();
    p1631_dfs_with_dp((0, 0), &heights, &mut trace, 0, &mut answers).unwrap()
}

pub fn p0802_eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cycled: Vec<bool> = vec![false; graph.len()];
    let mut terminated: Vec<bool> = vec![false; graph.len()];
    let mut skiped: HashSet<usize> = HashSet::new();

    for idx in 0..graph.len() {
        if skiped.contains(&idx) { continue; }

        let mut positions: HashSet<usize> = graph[idx].clone().into_iter().map(|n| n as usize).collect();
        while !cycled[idx] && !terminated[idx] {
            if positions.contains(&idx) {
                cycled[idx] = true;
            } else {
                for next in std::mem::replace(&mut positions, Default::default()).into_iter() {
                    if cycled[next] { cycled[idx] = true; break; }
                    for nn in graph[next].iter() { positions.insert(*nn as usize); }
                }
                if !cycled[idx] && positions.len() == 0 { terminated[idx] = true; }
            }
        }
        if cycled[idx] {
            for ndx in (idx+1)..graph.len() {
                if graph[ndx].iter().position(|n| *n == idx as i32).is_some() {
                    skiped.insert(idx);
                    cycled[idx] = true;
                }
            }
        }
        // eprintln!("{:?} - {:?} - {:?}", cycled, terminated, positions);
    }

    let mut res: Vec<_> = terminated.into_iter().enumerate().flat_map(|(idx, p)| if p { Some(idx as i32) } else { None }).collect();
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_set {
        ($left: expr, $right: expr) => {
            assert_eq!($left.len(), $right.len(), "length not match");
            let left: HashSet<_> = $left.into_iter().collect();
            let right: HashSet<_> = $right.into_iter().collect();
            assert_eq!(left, right);
        }
    }


    #[test]
    fn test_p0997() {
        assert_eq!(p0997_find_judge(1, vec![]), 1);
        assert_eq!(p0997_find_judge(2, vec![vec![1,2]]), 2);
        assert_eq!(p0997_find_judge(3, vec![vec![1,3], vec![2,3], vec![3,1]]), -1);
    }

    #[test]
    fn test_p0797() {
        assert_eq_set!(p0797_all_paths_source_target(vec![vec![2], vec![3], vec![1], vec![]]), vec![vec![0, 2, 1, 3]]);
        assert_eq_set!(p0797_all_paths_source_target(vec![vec![1,2], vec![3], vec![3], vec![]]), vec![vec![0, 1, 3], vec![0, 2, 3]]);
        assert_eq_set!(p0797_all_paths_source_target(vec![vec![1], vec![]]), vec![vec![0, 1]]);
        assert_eq_set!(p0797_all_paths_source_target(vec![vec![1,3], vec![2], vec![3], vec![]]), vec![vec![0,1,2,3], vec![0,3]]);
        assert_eq_set!(p0797_all_paths_source_target(vec![vec![1,2,3], vec![2], vec![3], vec![]]), vec![vec![0,1,2,3], vec![0,2,3], vec![0,3]]);
        assert_eq_set!(p0797_all_paths_source_target(vec![vec![4,3,1], vec![3,2,4], vec![3], vec![4], vec![]]), vec![vec![0,4], vec![0,3,4], vec![0,1,3,4], vec![0,1,2,3,4], vec![0,1,4]]);
    }

    #[test]
    fn test_p1557() {
        assert_eq_set!(p1557_find_smallest_set_of_vertices(6, vec![vec![0,1], vec![0,2], vec![2,5], vec![3,4], vec![4,2]]), vec![0, 3]);
        assert_eq_set!(p1557_find_smallest_set_of_vertices(5, vec![vec![0,1], vec![2,1], vec![3,1], vec![1,4], vec![2,4]]), vec![0, 2, 3]);
    }

    #[test]
    fn test_p1387() {
        assert_eq!(p1387_get_kth(12, 15, 2), 13);
        assert_eq!(p1387_get_kth(1, 1, 1), 1);
        assert_eq!(p1387_get_kth(7, 11, 4), 7);
        assert_eq!(p1387_get_kth(10, 20, 5), 13);
        assert_eq!(p1387_get_kth(1, 1000, 777), 570);
    }

    #[test]
    fn test_p0950() {
        assert_eq!(p0959_regions_by_slashes(vec![" /".into(), "/ ".into()]), 2);
        assert_eq!(p0959_regions_by_slashes(vec![" /".into(), "  ".into()]), 1);
        assert_eq!(p0959_regions_by_slashes(vec!["\\/".into(), "/\\".into()]), 4);
        assert_eq!(p0959_regions_by_slashes(vec!["/\\".into(), "\\/".into()]), 5);
        assert_eq!(p0959_regions_by_slashes(vec!["//".into(), "/ ".into()]), 3);
    }

    #[test]
    fn test_p0841() {
        assert_eq!(p0841_can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]), true);
        assert_eq!(p0841_can_visit_all_rooms(vec![vec![1,3], vec![3,0,1], vec![2], vec![0]]), false);
    }

    #[test]
    fn test_p0684() {
        assert_eq!(p0684_find_redundant_connection(vec![
                vec![2,7], vec![7,8], vec![3,6], vec![2,5], vec![6,8],
                vec![4,8], vec![2,8], vec![1,8], vec![7,10], vec![3,9]]), vec![2, 8]);
        assert_eq!(p0684_find_redundant_connection(vec![vec![1,3], vec![3,4], vec![1,5], vec![3,5], vec![2,3]]), vec![3, 5]);
        assert_eq!(p0684_find_redundant_connection(vec![vec![1,2], vec![1,3], vec![2,3]]), vec![2, 3]);
        assert_eq!(p0684_find_redundant_connection(vec![vec![1,2], vec![2,3], vec![3,4], vec![1,4], vec![1,5]]), vec![1, 4]);
    }

    #[test]
    fn test_p1267() {
        assert_eq!(p1267_count_servers(vec![vec![1,0,0,1,0], vec![0,0,0,0,0], vec![0,0,0,1,0]]), 3);
        assert_eq!(p1267_count_servers(vec![vec![1,0], vec![0,1]]), 0);
        assert_eq!(p1267_count_servers(vec![vec![1,0], vec![1,1]]), 3);
        assert_eq!(p1267_count_servers(vec![vec![1,1,0,0], vec![0,0,1,0], vec![0,0,1,0], vec![0,0,0,1]]), 4);
    }

    #[test]
    fn test_p0399() {
        assert_eq!(
            p0399_calc_equation(
                [["a","b"],["b","c"],["bc","cd"]].iter().map(|c| c.iter().map(|v| v.to_string()).collect()).collect(),
                vec![1.5,2.5,5.0],
                [["a","c"],["c","b"],["bc","cd"],["cd","bc"]].iter().map(|c| c.iter().map(|v| v.to_string()).collect()).collect(),
            ),
            vec![3.75000,0.40000,5.00000,0.20000]
        );
        assert_eq!(
            p0399_calc_equation(
                vec![vec!["a".into(),"b".into()]],
                vec![0.5],
                vec![vec!["a".into(),"b".into()], vec!["b".into(),"a".into()], vec!["a".into(),"c".into()], vec!["x".into(),"y".into()]]
            ),
            vec![0.50000,2.00000,-1.00000,-1.00000]
        );
        assert_eq!(
            p0399_calc_equation(
                vec![vec!["a".into(),"b".into()], vec!["b".into(),"c".into()]],
                vec![2.0,3.0],
                vec![vec!["a".into(),"c".into()], vec!["b".into(),"a".into()], vec!["a".into(),"e".into()], vec!["a".into(),"a".into()], vec!["x".into(),"x".into()]]
            ),
            vec![6.0, 0.5, -1.0, 1.0, -1.0]
        );
    }

    #[test]
    fn test_p1615() {
        assert_eq!(p1615_maximal_network_rank(4, [[0,1],[0,3],[1,2],[1,3]].iter().map(|s| s.into_iter().map(|s| *s as i32).collect()).collect()), 4);
        assert_eq!(p1615_maximal_network_rank(5, [[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]].iter().map(|s| s.into_iter().map(|s| *s as i32).collect()).collect()), 5);
        assert_eq!(p1615_maximal_network_rank(8, [[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]].iter().map(|s| s.into_iter().map(|s| *s as i32).collect()).collect()), 5);
    }

    #[test]
    fn test_p1631() {
        // assert_eq!(p1631_minimum_effort_path([
        //         [4,  3, 4,  10, 5,  5, 9, 2],
        //         [10, 8, 2,  10, 9,  7, 5, 6],
        //         [5,  8, 10, 10, 10, 7, 4, 2],
        //         [5,  1, 3,  1,  1,  3, 1, 9],
        //         [6,  4, 10, 6,  10, 9, 4, 6]
        // ].iter().map(|s| s.into_iter().map(|s| *s as i32).collect()).collect()), 2);
        // assert_eq!(p1631_minimum_effort_path([[1,2,3],[3,8,4],[5,3,5]].iter().map(|s| s.into_iter().map(|s| *s as i32).collect()).collect()), 1);
        // assert_eq!(p1631_minimum_effort_path([[1,2,2],[3,8,2],[5,3,5]].iter().map(|s| s.into_iter().map(|s| *s as i32).collect()).collect()), 2);
        // assert_eq!(p1631_minimum_effort_path([[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]].iter().map(|s| s.into_iter().map(|s| *s as i32).collect()).collect()), 0);
    }

    #[test]
    fn test_p0802() {
        // assert_eq!(
        //     p0802_eventual_safe_nodes(
        //     ),
        //     vec![2,4,5,6]
        // );
        assert_eq!(
            p0802_eventual_safe_nodes(
                vec![vec![1,2], vec![2,3], vec![5], vec![0], vec![5], vec![], vec![]],
            ),
            vec![2,4,5,6]
        );
        assert_eq!(
            p0802_eventual_safe_nodes(
                vec![vec![1,2,3,4], vec![1,2], vec![3,4], vec![0,4], vec![]]
            ),
            vec![4]
        );
    }
}

