use itertools::Itertools;

type Point = (u16, u16);
type Input<'b> = Vec<&'b [u8]>;

pub fn solve1(input: Input) -> usize {
    let mut distance: usize = 1;

    let start = find_start(&input);
    let (mut current_point, _) = find_next_segments(&input, start);
    let mut previous_point = start;

    while current_point != start {
        distance += 1;

        let (p1, p2) = connecting_segments(&input, current_point).unwrap();

        (current_point, previous_point) =
            (if p1 == previous_point { p2 } else { p1 }, current_point);
    }

    distance / 2
}

pub fn solve2(input: Input) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut polygon_raster: Vec<Vec<Option<u16>>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut vec = Vec::with_capacity(cols);
        for _ in 0..cols {
            vec.push(None);
        }

        polygon_raster.push(vec);
    }

    let start = find_start(&input);
    let (mut current_point, _) = find_next_segments(&input, start);
    let mut previous_point = start;
    let mut distance: u16 = 1;
    
    polygon_raster[start.0 as usize][start.1 as usize] = Some(0);

    while current_point != start {
        polygon_raster[current_point.0 as usize][current_point.1 as usize] = Some(distance);
        distance += 1;

        let (p1, p2) = connecting_segments(&input, current_point).unwrap();

        (current_point, previous_point) =
            (if p1 == previous_point { p2 } else { p1 }, current_point);
    }

    let polygon_length = distance;

    let mut winding_count = 0;
    let mut inside_segments = 0;

    // non-zero winding rule
    // https://en.wikipedia.org/wiki/Nonzero-rule
    for row_index in 0..rows {
        for col_index in 0..cols {
            if let Some(distance) = polygon_raster[row_index][col_index] {
                if row_index + 1 < rows {
                    if let Some(distance_below) = polygon_raster[row_index + 1][col_index] {
                        if distance_below == (distance + 1) % polygon_length {
                            winding_count += 1
                        }
                        if distance == (distance_below + 1) % polygon_length {
                            winding_count -= 1
                        }
                    }
                }
            } else {
                if winding_count != 0 {
                    inside_segments += 1
                }
            }
        }
    }

    inside_segments
}

fn find_next_segments(input: &Input, start_point: Point) -> (Point, Point) {
    let mut segments = Vec::with_capacity(2);

    for point in [
        (start_point.0 + 1, start_point.1 + 0),
        (start_point.0.checked_sub(1).unwrap_or(0), start_point.1 + 0),
        (start_point.0 + 0, start_point.1 + 1),
        (start_point.0 + 0, start_point.1.checked_sub(1).unwrap_or(0)),
    ] {
        if let Some((p1, p2)) = connecting_segments(&input, point) {
            if p1 == start_point || p2 == start_point {
                segments.push(point);
            }
        }
    }

    (segments[0], segments[1])
}

fn find_start(input: &Input) -> Point {
    for (row_index, row) in input.into_iter().enumerate() {
        if let Some((col_index, _)) = row.into_iter().find_position(|symbol| **symbol == b'S') {
            return (row_index as u16, col_index as u16);
        }
    }

    panic!("No starting position found!");
}

fn connecting_segments(input: &Input, (row, col): Point) -> Option<(Point, Point)> {
    match input[row as usize][col as usize] {
        b'|' if row > 0 => Some(((row - 1, col), (row + 1, col))),
        b'-' if col > 0 => Some(((row, col + 1), (row, col - 1))),
        b'L' if row > 0 => Some(((row - 1, col), (row, col + 1))),
        b'J' if row > 0 && col > 0 => Some(((row - 1, col), (row, col - 1))),
        b'7' if col > 0 => Some(((row + 1, col), (row, col - 1))),
        b'F' => Some(((row + 1, col), (row, col + 1))),
        _ => None,
    }
}

pub fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.as_bytes()).collect_vec()
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day10::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT_1: &'static str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

    const EXAMPLE_INPUT_2: &'static str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

    const EXAMPLE_INPUT_3: &'static str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    const EXAMPLE_INPUT_4: &'static str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT_1)), 4);
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT_2)), 4);
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT_3)), 8);
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT_4)), 8);
    }

    #[test]
    fn solve_part1() {
        let input = read_input(10, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str())));
    }

    const EXAMPLE_INPUT_5: &'static str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

    const EXAMPLE_INPUT_6: &'static str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

    const EXAMPLE_INPUT_7: &'static str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

    const EXAMPLE_INPUT_8: &'static str = r#"...........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#;

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT_5)), 4);
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT_8)), 4);
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT_6)), 8);
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT_7)), 10);
    }

    #[test]
    fn solve_part2() {
        let input = read_input(10, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str())))
    }
}
