use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
struct Point(usize, usize);

fn heuristic(a: Point, b: Point) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

fn a_star(maze: &Vec<Vec<i32>>, start: Point, goal: Point) -> Option<usize> {
    let directions = [(0,1), (1,0), (0,-1), (-1,0)];
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut parent = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(Reverse((heuristic(start, goal), start)));

    while let Some(Reverse((_, current))) = open_set.pop() {
        //println!("Visiting: {:?} with priority {}", current, priority);
        if current == goal {
            //println!("Goal reached! Distance: {}", g_score[&goal]);
            return Some(*g_score.get(&goal).unwrap());
        }
        
        for &(dx, dy) in &directions {
            let new_x = current.0 as isize + dx;
            let new_y = current.1 as isize + dy;
            
            if new_x >= 0 && new_y >= 0 {
                let new_pos = Point(new_x as usize, new_y as usize);
                
                if new_pos.0 < maze.len() && new_pos.1 < maze[0].len() && maze[new_pos.0][new_pos.1] == 1 {
                    let new_cost = g_score[&current] + 1;
                    
                    if !g_score.contains_key(&new_pos) || new_cost < g_score[&new_pos] {
                        //println!("Updating {:?} with new cost {}", new_pos, new_cost);
                        g_score.insert(new_pos, new_cost);
                        let priority = new_cost + heuristic(new_pos, goal);
                        open_set.push(Reverse((priority, new_pos)));
                        parent.insert(new_pos, current);
                    }
                }
            }
        }
    }
    None
}

fn find_shortest_path(maze: &Vec<Vec<i32>>, entry: Point, exit: Point, touchpoints: (Point, Point)) -> Option<usize> {
    let (tp1, tp2) = touchpoints;
    let path_a = a_star(maze, entry, tp1)?;
    let path_b = a_star(maze, tp1, tp2)?;
    let path_c = a_star(maze, tp2, exit)?;
    //println!("{},{},{}", path_a, path_b, path_c);
    return Some(path_a + path_b + path_c);
}

fn maze_to_string(maze: &Vec<Vec<i32>>) -> String {
    maze.iter()
        .flat_map(|row| row.iter().map(|&cell| if cell == 1 { '.' } else { '#' }))
        .collect()
}

fn optimize_maze(
    maze: &mut Vec<Vec<i32>>,
    entry: Point,
    exit: Point,
    touchpoints: (Point, Point),
    best_length: &mut usize,
    best_maze: &mut Vec<Vec<i32>>,
    cache: &mut HashSet<String>
) {
    let current_maze_hash = maze_to_string(maze);
    if cache.contains(&current_maze_hash) {
        return;  // Skip already seen mazes
    }
    cache.insert(current_maze_hash);

    let current_length = find_shortest_path(maze, entry, exit, touchpoints).unwrap_or(usize::MIN);

    (0..maze.len()).for_each(|x| {
        (0..maze[0].len()).for_each(|y| {
            if maze[x][y] == 0 {
                return;
            }

            maze[x][y] = 0;
            let new_length = find_shortest_path(maze, entry, exit, touchpoints).unwrap_or(usize::MIN);

            if new_length < current_length {
                maze[x][y] = 1;
                return;
            } else {
                if new_length > *best_length {
                    *best_length = new_length;
                    *best_maze = maze.clone();
                }
                optimize_maze(maze, entry, exit, touchpoints, best_length, best_maze, cache);
                maze[x][y] = 1;
            }
        })
    })
}

fn print_maze(entry: Point, exit: Point, touchpoints: (Point, Point), best_maze: Vec<Vec<i32>>) {
    for (i, row) in best_maze.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let cell_point = Point(i, j);
            if cell_point == entry || cell_point == exit {
                print!("o");
            } else if cell_point == touchpoints.0 {
                print!("1");
            } else if cell_point == touchpoints.1 {
                print!("2");
            } else {
                print!("{}", if cell == 1 { '.' } else { '#' });
            }
        }
        println!();
    }
}

fn main() {
    // let rows = 15;
    // let cols = 19;
    // let entry = Point(0, 4);
    // let exit = Point(0, 14);
    // let touchpoints = (Point(10, 4), Point(10, 14));

    let rows = 3;
    let cols = 3;
    let entry = Point(0,0);
    let exit = Point(0,2);
    let touchpoints = (Point(2,1), Point(2,1));
    
    let mut maze = vec![vec![1; cols]; rows];
    let mut best_maze = maze.clone();
    let mut best_length = find_shortest_path(&maze, entry, exit, touchpoints).unwrap_or(0);
    let mut cache = HashSet::new();
    
    optimize_maze(&mut maze, entry, exit, touchpoints, &mut best_length, &mut best_maze, &mut cache);
    print_maze(entry, exit, touchpoints, best_maze);
    println!("Best Path Length: {}", best_length);
}