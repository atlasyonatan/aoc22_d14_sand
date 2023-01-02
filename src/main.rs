use ndarray::Array2;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod point2;
use crate::point2::Point2;
type Point = Point2<usize>;
type Line = (Point, Point);

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let rock_formations: Vec<Line> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|l| -> Vec<Line> {
            let vec: Vec<Point> = l
                .split("->")
                .map(|s| {
                    let mut iter = s.trim().split(',').map(|s| s.parse().unwrap());
                    Point {
                        x: iter.next().unwrap(),
                        y: iter.next().unwrap(),
                    }
                })
                .collect();
            (0..(vec.len() - 1))
                .map(|i| (vec[i].clone(), vec[i + 1].clone()))
                .collect()
        })
        .collect();
    let source = Point { x: 500, y: 0 };
    let (mut grid, offset) = {
        let min = Point {
            x: rock_formations
                .iter()
                .flat_map(|line| [line.0.x, line.1.x])
                .chain([source.x])
                .min()
                .unwrap(),
            y: rock_formations
                .iter()
                .flat_map(|line| [line.0.y, line.1.y])
                .chain([source.y])
                .min()
                .unwrap(),
        };
        let max = Point {
            x: rock_formations
                .iter()
                .flat_map(|line| [line.0.x, line.1.x])
                .chain([source.x])
                .max()
                .unwrap(),
            y: rock_formations
                .iter()
                .flat_map(|line| [line.0.y, line.1.y])
                .chain([source.y])
                .max()
                .unwrap(),
        };
        let offset = min;
        let size = max - offset + Point { x: 1, y: 1 };
        let mut grid = Array2::from_elem((size.x, size.y), None);
        for (start, end) in rock_formations {
            let start = start - offset;
            let end = end - offset;
            for x in start.x.min(end.x)..=start.x.max(end.x) {
                for y in start.y.min(end.y)..=start.y.max(end.y) {
                    grid[[x, y]] = Some(Material::Rock);
                }
            }
        }
        (grid, min)
    };

    let source = source - offset;

    let mut display_grid = grid.map(|o| match o {
        Some(Material::Rock) => '#',
        Some(Material::Sand) => 'o',
        None => '.',
    });
    display_grid[[source.x, source.y]] = '+';

    let moves = [
        Point2 { x: 0, y: 1 },
        Point2 { x: -1, y: 1 },
        Point2 { x: 1, y: 1 },
    ];
    //part 1{
    let sand_start = Point2 {
        x: source.x as i32,
        y: source.y as i32,
    };
    let grid_x_range = 0..(grid.dim().0 as i32);
    let grid_y_range = 0..(grid.dim().1 as i32);
    let mut resting_sand = 0;
    'simulate: loop {
        let mut sand_position = sand_start.clone();
        loop {
            let possible_position = moves
                .iter()
                .map(|&step| sand_position + step)
                .map(|p| {
                    (
                        p,
                        match grid_x_range.contains(&p.x) && grid_y_range.contains(&p.y) {
                            true => match grid[[p.x as usize, p.y as usize]] {
                                Some(_) => MoveResult::Occupied,
                                None => MoveResult::Vacant,
                            },
                            false => MoveResult::Outside,
                        },
                    )
                })
                .find(|(_, result)| match result {
                    MoveResult::Occupied => false,
                    _ => true,
                });
            match possible_position {
                Some((new_position, result)) => match result {
                    MoveResult::Outside => {
                        break 'simulate;
                    }
                    _ => {
                        //move sand to new_position
                        sand_position = new_position;
                    }
                },
                None => {
                    //solidify sand at current position
                    grid[[sand_position.x as usize, sand_position.y as usize]] =
                        Some(Material::Sand);
                    resting_sand += 1;
                    break;
                }
            }
        }
    }
    let mut display_grid = grid.map(|o| match o {
        Some(Material::Rock) => '#',
        Some(Material::Sand) => 'o',
        None => '.',
    });
    display_grid[[source.x, source.y]] = '+';

    println!("{}", display_char_grid(&display_grid));
    println!("part 1: {}", resting_sand);
}

fn display_char_grid(grid: &Array2<char>) -> String {
    let mut string = String::new();
    for row in grid.columns() {
        string.extend(row);
        string.push('\n');
    }
    string.pop();
    string
}

#[derive(Debug, Clone)]
enum Material {
    Sand,
    Rock,
}

enum MoveResult {
    Occupied,
    Vacant,
    Outside,
}
