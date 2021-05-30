
pub type DijkstraGraph = Vec<Vec<usize>>;

fn dijkstra_add_edge(graph: &mut DijkstraGraph, src: usize, dst: usize, weight: usize) {
    graph[src][dst] = weight;
}

fn dijkstra_min(mdist: &Vec<usize>, walked: &Vec<bool>) -> usize {
    let mut min = std::usize::MAX;
    let mut minid = 0;
    for u in 0..mdist.len() {
        if !walked[u] && mdist[u] < min {
            min = mdist[u];
            minid = u;
        }
    }
    minid
}



pub fn dijkstra_solve(graph: &mut DijkstraGraph, src: usize) -> usize {
    let N = graph.len();
    let mut mdist = vec![0; N];
    let mut walked = vec![false; N];

    mdist[src] = 0;
    for _ in 0..N {
        let u = dijkstra_min(&mdist, &walked);
        walked[u] = true;

        for v in 0..N {
            if !walked[v] && graph[u][v] > 0 && mdist[u] + graph[u][v] < mdist[v] {
                mdist[v] = mdist[u] + graph[u][v];
            }
        }
    }

    let u = dijkstra_min(&mdist, &walked);

    let u = dijkstra_min(&mdist, &walked);
    mdist[u]
}

pub fn can_jump(nums: &Vec<i32>) -> bool {
    let mut can_arrived = vec![false; nums.len()];
    can_arrived[nums.len() - 1] = true;

    for u in (0..nums.len()).rev() {
        let mut is_arrived = can_arrived[u];
        for v in u+1..u+nums[u] as usize+1 {
            if v < nums.len() && can_arrived[v] {
                is_arrived = true;
                break;
            }
        }

        can_arrived[u] = is_arrived;
    }
    // println!("{:?}", can_arrived);

    can_arrived[0]
}

pub fn can_jump_faster(nums: &Vec<i32>) -> bool {
    // let mut last_pointer = nums.len() - 1;
    // for u in (0..nums.len()).rev() {
    //     if u + nums[u] as usize >= last_pointer {
    //         last_pointer = u;
    //     }
    // }
    // last_pointer == 0

    let mut last_pointer = 0;
    for u in 0..nums.len() {
        if u <= last_pointer && u + nums[u] as usize > last_pointer {
            last_pointer = u + nums[u] as usize
        }
        if last_pointer + 1 >= nums.len() { return true; }
    }
    false
}

pub fn knapsack_sovle(mut items: Vec<(usize, usize)>, mut capacity: f64) -> f64 {
    items.sort_by(|(w1, p1), (w2, p2)| {
        (*p1 as f64 / *w1 as f64).partial_cmp(&(*p2 as f64 / *w2 as f64)).unwrap().reverse()
    });

    let mut maxprofit = 0.;
    for &(weight, profit)  in items.iter() {
        if weight as f64 <= capacity {
            maxprofit += profit as f64;
            capacity -= weight as f64;
        } else {
            break
        }
    }
    maxprofit
}

pub fn kruskals_mini_spanning_tree<const N: usize>(graph: &[[usize; N]; N]) {
    for i in 0..N {
        let mut min = std::usize::MAX;
        let mut idx = 0;
        for j in 0..N {
            if graph[i][j] > 0 && graph[i][j] < min {
                min = graph[i][j];
                idx = j;
            }
        }

        println!("{} - {} \t {}", i, idx, graph[i][idx]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_sample() {
        let N = 10;
        let mut graph = vec![vec![0; N]; N];

        // assert_eq!(dijkstra_solve(&mut graph, 0), 0);
    }

    #[test]
    fn can_jump_sample() {
        assert_eq!(can_jump(&vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump(&vec![3, 2, 1, 0, 4]), false);
        assert_eq!(can_jump_faster(&vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump_faster(&vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn kruskals_sample() {
        const INFINITY: usize = std::usize::MAX;
        let graph = [
            [        0,        4,        1,        4,        INFINITY, INFINITY, ],
            [        4,        0,        3,        8,        3,        INFINITY, ],
            [        1,        3,        0,        INFINITY, 1,        INFINITY, ],
            [        4,        8,        INFINITY, 0,        5,        7,        ],
            [        INFINITY, 3,        1,        5,        0,        INFINITY, ],
            [        INFINITY, INFINITY, INFINITY, 7,        INFINITY, 0,        ],
        ];
        kruskals_mini_spanning_tree(&graph);
    }



}
