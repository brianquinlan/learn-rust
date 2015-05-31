// Reference: http://www.redblobgames.com/pathfinding/a-star/introduction.html

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

// Used to maintain a priority queue of positions to explore.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PositionPriority {
    minimum_cost: usize,
    position: Position,
}

// The priority queue depends on `Ord`.
impl Ord for PositionPriority {
    // Reverse the comparison so the position with the lowest cost is
    // explored first.
    fn cmp(&self, other: &PositionPriority) -> Ordering {
        other.minimum_cost.cmp(&self.minimum_cost)
    }
}

impl PartialOrd for PositionPriority {
    fn partial_cmp(&self, other: &PositionPriority) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
fn min_distance_heuristic(p1: Position, p2: Position) -> usize {
    let x_diff = match p1.x > p2.x {
        true => p1.x - p2.x,
        false => p2.x - p1.x,
    };
    let y_diff = match p1.y > p2.y {
        true => p1.y - p2.y,
        false => p2.y - p1.y,
    };

    x_diff + y_diff
}

#[inline]
fn neighbours(grid: &Vec<Vec<char>>, pos: Position) -> Vec<Position> {
    let mut v = Vec::new();

    if pos.x > 0 && grid[pos.y][pos.x-1] == ' ' {
        v.push(Position{x: pos.x-1, y: pos.y});
    }
    if pos.x + 1 < grid[pos.y].len() && grid[pos.y][pos.x+1] == ' ' {
        v.push(Position{x: pos.x+1, y: pos.y});
    }

    if pos.y > 0 && pos.x < grid[pos.y-1].len() && grid[pos.y-1][pos.x] == ' ' {
        v.push(Position{x: pos.x, y: pos.y-1});
    }
    if pos.y + 1 < grid.len() &&
       pos.x < grid[pos.y+1].len() &&
       grid[pos.y+1][pos.x] == ' ' {
        v.push(Position{x: pos.x, y: pos.y+1});
    }
    v
}

/// Generate the optimal path from start to goal given a mapping from positions
/// to the position that lead to that position.
#[inline]
fn reconstruct(came_from: &HashMap<Position, Position>,
               start: Position,
               goal: Position) -> Vec<Position> {
    let mut current = goal;
    let mut path = vec![current];
    while current != start {
        current = came_from[&current];
        path.push(current);
    }
    path.reverse();
    path
}

/// Find the optimal path from a starting position to a goal position. Return
/// None if no path is possible. The input grid is a 2-dimensional grid of
/// characters. Any character other than a space is considered an impassable
/// obstacle.
///
/// # Examples
///
/// ```
/// use bestpath::{find_path, Position};
///
/// let grid = vec![vec![' ', '*', '*', ' '],
///                 vec![' ', ' ', ' ', '*'],
///                 vec!['*', '*', ' ', ' '],
///                 vec![' ', '*', ' ', ' ']];
/// assert_eq!(Some(vec![Position{x: 0, y: 0},
///                      Position{x: 0, y: 1},
///                      Position{x: 1, y: 1},
///                      Position{x: 2, y: 1},
///                      Position{x: 2, y: 2}]),
///            find_path(&grid, Position{x: 0, y: 0}, Position{x: 2, y: 2}));
/// assert_eq!(None,
///            find_path(&grid, Position{x: 0, y: 0}, Position{x: 3, y: 0}));
/// ```
#[inline]
pub fn find_path(grid: &Vec<Vec<char>>, start: Position, goal: Position) 
        -> Option<Vec<Position>>  {
    let mut frontier = BinaryHeap::new();
    frontier.push(PositionPriority { minimum_cost: 0, position: start });

    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some(PositionPriority { minimum_cost, position }) =
            frontier.pop() {
        if position == goal { 
            return Some(reconstruct(&came_from, start, goal));
        }
        let cost = cost_so_far[&position];

        for next_position in neighbours(grid, position) {
            let new_cost = cost + 1;
            if let Some(existing_cost) = cost_so_far.get(&next_position) {
                if new_cost >= *existing_cost {
                    continue;
                }
            }
            cost_so_far.insert(next_position, new_cost);
            frontier.push(PositionPriority { 
                minimum_cost: new_cost +
                    min_distance_heuristic(goal, next_position),
                position: next_position });
            came_from.insert(next_position, position);
        }
    }
    None
}

/// Generate a copy of the given grid with the path filled in. The starting
/// position is shown with a '@' , the goal postiion with a 'X' and all other
/// points on the path with a 'o'.
///
/// # Examples
///
/// ```
/// use bestpath::{find_path, format_path_map, Position};
///
/// let grid = vec![vec![' ', ' ', ' ', ' '],
///                 vec![' ', '*', ' ', '*'],
///                 vec!['*', '*', ' ', ' '],
///                 vec![' ', '*', ' ', ' ']];
/// let path = find_path(&grid,
///                      Position{x: 0, y: 1},
///                      Position{x: 3, y: 2}).unwrap();
/// assert_eq!(vec![vec!['o', 'o', 'o', ' '],
///                 vec!['@', '*', 'o', '*'],
///                 vec!['*', '*', 'o', 'X'],
///                 vec![' ', '*', ' ', ' ']],
///            format_path_map(&grid, &path));
/// ```
#[inline]
pub fn format_path_map(grid: &Vec<Vec<char>>, path: &Vec<Position>)
        -> Vec<Vec<char>> {
    let mut m = grid.clone();
    let last_path = path.len() - 1;
    for (i, p) in path.iter().enumerate() {
        m[p.y][p.x] = match i {
            0 => '@',
            _ if i == last_path => 'X',
            _ => 'o',
        }
    }
    m
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    println!("");
    for row in grid {
        for ch in row {
            print!(" {} ", ch);
        }
        println!("");
    }
}

#[test]
fn no_path() {
    let map =
        vec![vec![' ', ' ', '█', ' ', ' ', ' ', ' '],
             vec!['█', '█', ' ', ' ', '█', '█', ' '],
             vec![' ', '█', ' ', '█', '█', ' ', '█'],
             vec![' ', ' ', ' ', '█', ' ', '█', ' '],
             vec!['█', ' ', ' ', ' ', ' ', '█', ' '],
             vec![' ', '█', '█', '█', '█', ' ', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' ']];

    let path = find_path(&map, Position { x: 0, y: 0}, Position { x: 6, y: 6});
    assert_eq!(None, path);
}

#[test]
fn multiple_complex_paths_to_goal() {
    let map =
        vec![vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec!['█', '█', ' ', '█', '█', ' ', ' '],
             vec![' ', '█', ' ', '█', '█', ' ', '█'],
             vec![' ', '█', ' ', '█', ' ', ' ', ' '],
             vec![' ', '█', ' ', '█', '█', ' ', '█'],
             vec![' ', '█', '█', ' ', ' ', ' ', ' '],
             vec!['█', '█', ' ', ' ', '█', '█', ' '],
             vec![' ', ' ', ' ', ' ', '█', '█', ' '],
             vec![' ', ' ', ' ', ' ', '█', ' ', ' ']];

    let path = find_path(&map,
                         Position { x: 0, y: 0},
                         Position { x: 6, y: 8}).unwrap();
    let path_map = format_path_map(&map, &path);
    print_grid(&path_map);
    assert_eq!(
        vec![vec!['@', 'o', 'o', 'o', 'o', 'o', ' '],
             vec!['█', '█', ' ', '█', '█', 'o', ' '],
             vec![' ', '█', ' ', '█', '█', 'o', '█'],
             vec![' ', '█', ' ', '█', ' ', 'o', ' '],
             vec![' ', '█', ' ', '█', '█', 'o', '█'],
             vec![' ', '█', '█', ' ', ' ', 'o', 'o'],
             vec!['█', '█', ' ', ' ', '█', '█', 'o'],
             vec![' ', ' ', ' ', ' ', '█', '█', 'o'],
             vec![' ', ' ', ' ', ' ', '█', ' ', 'X']],
        path_map);
}

#[test]
fn verticle_map() {
    let map =
        vec![vec![' '],
             vec![' '],
             vec![' '],
             vec![' ']];

    let path = find_path(&map,
                         Position { x: 0, y: 0},
                         Position { x: 0, y: 3}).unwrap();
    let path_map = format_path_map(&map, &path);
    print_grid(&path_map);
    assert_eq!(
        vec![vec!['@'],
             vec!['o'],
             vec!['o'],
             vec!['X']],
        path_map);
}

#[test]
fn horizontal_map() {
    let map =
        vec![vec![' ', ' ', ' ', ' ']];

    let path = find_path(&map,
                         Position { x: 0, y: 0},
                         Position { x: 3, y: 0}).unwrap();
    let path_map = format_path_map(&map, &path);
    print_grid(&path_map);
    assert_eq!(
        vec![vec!['@', 'o', 'o', 'X']],
        path_map);
}

#[test]
fn straight_diagonal_to_goal() {
    let map =
        vec![vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' ']];

    let path = find_path(&map,
                         Position { x: 0, y: 0},
                         Position { x: 6, y: 6}).unwrap();
    let path_map = format_path_map(&map, &path);
    print_grid(&path_map);
    assert_eq!(13, path.len())  // There are many equivalent paths.
}

#[test]
fn single_walled_path_to_goal() {
    let map =
        vec![vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec!['█', '█', '█', '█', '█', '█', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', '█', '█', '█', '█', '█', '█'],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec!['█', '█', '█', '█', '█', '█', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' ']];

    let path = find_path(&map,
                         Position { x: 0, y: 0},
                         Position { x: 0, y: 6}).unwrap();
    let path_map = format_path_map(&map, &path);
    print_grid(&path_map);
    assert_eq!(
        vec![vec!['@', 'o', 'o', 'o', 'o', 'o', 'o'],
             vec!['█', '█', '█', '█', '█', '█', 'o'],
             vec!['o', 'o', 'o', 'o', 'o', 'o', 'o'],
             vec!['o', '█', '█', '█', '█', '█', '█'],
             vec!['o', 'o', 'o', 'o', 'o', 'o', 'o'],
             vec!['█', '█', '█', '█', '█', '█', 'o'],
             vec!['X', 'o', 'o', 'o', 'o', 'o', 'o']],
        path_map);
}

#[test]
fn spiral_wall_path_to_goal() {
    let map =
        vec![vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', '█', '█', '█', '█', '█', '█', '█'],
             vec![' ', '█', ' ', ' ', ' ', ' ', ' ', ' '],
             vec![' ', '█', ' ', '█', '█', '█', '█', ' '],
             vec![' ', '█', ' ', '█', ' ', ' ', '█', ' '],
             vec![' ', '█', ' ', '█', '█', ' ', '█', ' '],
             vec![' ', '█', ' ', ' ', ' ', ' ', '█', ' '],
             vec![' ', '█', '█', '█', '█', '█', '█', ' '],
             vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']];

    let path = find_path(&map,
                         Position { x: 0, y: 0},
                         Position { x: 4, y: 4}).unwrap();
    let path_map = format_path_map(&map, &path);
    print_grid(&path_map);
    assert_eq!(
        vec![vec!['@', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
             vec!['o', '█', '█', '█', '█', '█', '█', '█'],
             vec!['o', '█', 'o', 'o', 'o', 'o', 'o', 'o'],
             vec!['o', '█', 'o', '█', '█', '█', '█', 'o'],
             vec!['o', '█', 'o', '█', 'X', 'o', '█', 'o'],
             vec!['o', '█', 'o', '█', '█', 'o', '█', 'o'],
             vec!['o', '█', 'o', 'o', 'o', 'o', '█', 'o'],
             vec!['o', '█', '█', '█', '█', '█', '█', 'o'],
             vec!['o', 'o', 'o', 'o', 'o', 'o', 'o', 'o']],
        path_map);
}
