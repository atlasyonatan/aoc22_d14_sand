use ndarray::Array2;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod point2;
use crate::point2::Point2;

pub mod simulation;
use crate::simulation::{display_char_grid, simulate_sand, Material};

fn main() {
    //read input as rock formations
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let mut rock_formations: Vec<(Point2<usize>, Point2<usize>)> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|l| -> Vec<(Point2<usize>, Point2<usize>)> {
            let vec: Vec<Point2<usize>> = l
                .split("->")
                .map(|s| {
                    let mut iter = s.trim().split(',').map(|s| s.parse().unwrap());
                    Point2 {
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
    let source = Point2 { x: 500, y: 0 };

    //add floor as rock formation
    let max_y = rock_formations
        .iter()
        .flat_map(|line| [line.0.y, line.1.y])
        .chain([source.y])
        .max()
        .unwrap();
    let floor_center = source + Point2 { x: 0, y: max_y + 2 };
    let radius = Point2 {
        x: floor_center.y,
        y: 0,
    };
    rock_formations.push((floor_center - radius, floor_center + radius));

    //construct grid with rock formations
    let min = Point2 {
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
    let max = Point2 {
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
    let size = max - min + Point2 { x: 1, y: 1 };
    let input_offset = min;

    //fill grid with input cave formations
    let mut grid = Array2::from_elem((size.x, size.y), None);
    for (start, end) in rock_formations {
        let start = start - input_offset;
        let end = end - input_offset;
        for x in start.x.min(end.x)..=start.x.max(end.x) {
            for y in start.y.min(end.y)..=start.y.max(end.y) {
                grid[[x, y]] = Some(Material::Rock);
            }
        }
    }
    let source = source - input_offset;

    //simulate
    simulate_sand(&mut grid, &source, floor_center.y as i32 - 1);

    //display grid after simulation
    let mut display_grid = grid.map(|o| match o {
        Some(Material::Rock) => '#',
        Some(Material::Sand) => 'o',
        None => '.',
    });
    display_grid[[source.x, source.y]] = '+';
    println!("{}", display_char_grid(&display_grid));
}
