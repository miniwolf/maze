use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
mod maze;

use maze::*;

fn heuristic(a: Point, b: Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn a_star(maze: Maze, start: Point, goal: Point) -> Option<usize> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
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
                if !maze.in_bounds(new_pos) {
                    continue;
                }

                if maze[new_pos] == Space {
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

fn find_shortest_path(maze: Maze, entry: Point, exit: Point, touchpoints: (Point, Point)) -> Option<usize> {
    let (tp1, tp2) = touchpoints;
    let path_a = a_star(maze, entry, tp1)?;
    let path_b = a_star(maze, tp1, tp2)?;
    let path_c = a_star(maze, tp2, exit)?;
    //println!("{},{},{}", path_a, path_b, path_c);
    Some(path_a + path_b + path_c)
}

fn optimize_maze(
    maze: Maze,
    entry: Point,
    exit: Point,
    touchpoints: (Point, Point),
    best_length: &mut usize,
    best_maze: &mut Maze,
) {
    let mut stack = VecDeque::new();
    let mut cache = HashSet::new();
    stack.push_back(maze.clone()); // Start with the initial maze

    while let Some(mut current_maze) = stack.pop_back() {
        let current_maze_hash = current_maze.to_string();
        if cache.contains(&current_maze_hash) {
            continue; // Skip already seen mazes
        }
        cache.insert(current_maze_hash);

        let current_length = find_shortest_path(current_maze, entry, exit, touchpoints)
            .unwrap_or(usize::MIN);

        for (x, y, value) in maze.iter() {
            let point = Point(x, y);
            if value == Wall {
                continue;
            }

            current_maze[point] = Wall;
            let new_length = find_shortest_path(current_maze, entry, exit, touchpoints)
                .unwrap_or(usize::MIN);

            if new_length < current_length {
                current_maze[point] = Space; // Restore state for next iteration
                continue;
            }

            if new_length > *best_length {
                // TODO: print here when a better one comes up. Shows progress
                *best_length = new_length;
                *best_maze = current_maze.clone();
            }

            stack.push_back(current_maze.clone()); // Push the modified maze for further processing
            current_maze[point] = Space; // Restore state for next iteration
        }
    }
}

fn print_maze(entry: Point, exit: Point, touchpoints: (Point, Point), best_maze: Maze) {
    for x in 0..best_maze.rows {
        for y in 0..best_maze.cols {
            let cell_point = Point(x, y);
            let cell = best_maze[cell_point];
            if cell_point == entry || cell_point == exit {
                print!("o");
            } else if cell_point == touchpoints.0 {
                print!("1");
            } else if cell_point == touchpoints.1 {
                print!("2");
            } else {
                print!("{}", cell.as_ref());
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

    // let rows = 3;
    // let cols = 3;
    // let entry = Point(0,0);
    // let exit = Point(0,2);
    // let touchpoints = (Point(2,1), Point(2,1));

    let rows = 4;
    let cols = 4;
    let entry = Point(0, 0);
    let exit = Point(0, 3);
    let touchpoints = (Point(2, 1), Point(2, 3));

    let maze = Maze::new(rows, cols, Space);
    let mut best_maze = maze;
    let mut best_length = find_shortest_path(maze, entry, exit, touchpoints).unwrap_or(0);

    optimize_maze(maze, entry, exit, touchpoints, &mut best_length, &mut best_maze);
    print_maze(entry, exit, touchpoints, best_maze);
    println!("Best Path Length: {}", best_length);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let rows = 4;
        let cols = 4;
        let entry = Point(0, 0);
        let exit = Point(0, 3);
        let touchpoints = (Point(2, 1), Point(2, 3));

        let maze = Maze::new(rows, cols, Space);
        let mut best_maze = maze;
        let mut best_length = find_shortest_path(maze, entry, exit, touchpoints).unwrap_or(0);
        assert_eq!(best_length, 7);

        optimize_maze(maze, entry, exit, touchpoints, &mut best_length, &mut best_maze);
        assert_eq!(best_maze.to_string(), ".....#.#..#.#...");
        assert_eq!(best_length, 17);
    }
}
