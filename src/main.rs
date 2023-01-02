use ndarray::Array2;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod point2;
type Point = point2::Point2<usize>;
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
            let start = start - min;
            let end = end - min;
            for x in start.x..=end.x {
                for y in start.y..=end.y {
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

    println!("{}", display_char_grid(&display_grid));
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
