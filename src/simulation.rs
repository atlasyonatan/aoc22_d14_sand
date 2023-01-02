use crate::point2::Point2;
use ndarray::Array2;

#[derive(Debug, Clone)]
pub enum Material {
    Sand,
    Rock,
}

enum MoveResult {
    Occupied,
    Vacant,
    Outside,
}

pub fn display_char_grid(grid: &Array2<char>) -> String {
    let mut string = String::new();
    for row in grid.columns() {
        string.extend(row);
        string.push('\n');
    }
    string.pop();
    string
}

pub fn simulate_sand(grid: &mut Array2<Option<Material>>, source: &Point2<usize>, on_floor_y: i32) {
    //sand moves in order of priority
    let moves = [
        Point2 { x: 0, y: 1 },
        Point2 { x: -1, y: 1 },
        Point2 { x: 1, y: 1 },
    ];
    let sand_start_position = Point2 {
        x: source.x as i32,
        y: source.y as i32,
    };
    let grid_bounds = Point2 {
        x: 0..(grid.dim().0 as i32),
        y: 0..(grid.dim().1 as i32),
    };

    //simulation tracking
    let mut resting_sand = 0;
    let mut clean_floor = true;

    //run simulation
    'simulate: loop {
        let mut sand_position = sand_start_position.clone();
        loop {
            //find first possible move (theoretial new position, the type of result the move will cause)
            let possible_position = moves
                .iter()
                .map(|&step| sand_position + step)
                .map(|p| {
                    (
                        p,
                        match grid_bounds.x.contains(&p.x) && grid_bounds.y.contains(&p.y) {
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
            //decide the sand's fate according to the found move
            match possible_position {
                Some((new_position, move_result)) => match move_result {
                    MoveResult::Outside => {
                        println!(
                            "After {} resting sand particles: sand overflows to abyss",
                            resting_sand
                        );
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
                    if sand_position == sand_start_position {
                        println!(
                            "After {} resting sand particles: source is plugged by resting sand",
                            resting_sand
                        );
                        break 'simulate;
                    }
                    break;
                }
            }
            //check first time floor is reached
            if clean_floor && sand_position.y == on_floor_y {
                clean_floor = false;
                println!(
                    "After {} resting sand particles: sand pours on the floor",
                    resting_sand
                );
            }
        }
    }
}
