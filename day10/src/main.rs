use std::ops::Add;
use std::ops::AddAssign;

fn add_margin(s: &str) -> String {
    let mut margin_maze = String::new();
    let mut lines = s.lines();

    let first_line = lines.next().unwrap();
    let len = first_line.len();

    let blank_line = &".".repeat(len + 2);
    // add top margin
    margin_maze.push_str(blank_line);
    margin_maze.push('\n');

    // add left and right margins to first line
    let first_line_padded = format!(".{}.", first_line);
    margin_maze.push_str(&first_line_padded);
    margin_maze.push('\n');

    // add left and right margins to each line
    for line in lines {
        let padded_line = format!(".{}.", line);
        margin_maze.push_str(&padded_line);
        margin_maze.push('\n');
    }

    // add bottom margin
    margin_maze.push_str(blank_line);
    margin_maze.push('\n');

    margin_maze
}

#[derive(Clone)]
struct PipeMaze {
    maze: Vec<Vec<PipeSection>>,
    start: (usize, usize),
}

impl PipeMaze {
    /// Returns a new maze with the given location marked with an X
    #[allow(dead_code)]
    fn with_location(&self, (row, col): (usize, usize)) -> Self {
        let mut new_maze = self.maze.clone();
        new_maze[row][col] = PipeSection::Marker;
        Self {
            maze: new_maze,
            start: self.start,
        }
    }
}

impl std::str::FromStr for PipeMaze {
    type Err = ();

    fn from_str(s: &str) -> Result<PipeMaze, Self::Err> {
        let maze = Self::read_maze(s);
        let start = Self::find_start(&maze);
        Ok(PipeMaze { maze, start })
    }
}

impl std::fmt::Display for PipeMaze {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.maze {
            for col in row {
                if f.alternate() {
                    write!(f, "{:#}", col)?;
                } else {
                    write!(f, "{}", col)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PipeMaze {
    /// Returns the PipeSection type at the given location
    pub fn pipe_section_at(&self, (row, col): (usize, usize)) -> PipeSection {
        self.maze[row][col]
    }

    /// Returns the two directions that the start position has "exits" to
    fn start_exit_directions(&self) -> (Direction, Direction) {
        use Direction::*;
        let mut next_dir = Some(North);
        let mut first = None;
        let mut second = None;

        while let Some(cur_dir) = next_dir {
            let neighbor = self.start + cur_dir;
            let neighbor_pipe = self.pipe_section_at(neighbor);

            if neighbor_pipe.has_entrance_from(cur_dir.flip()) {
                if first.is_none() {
                    first = Some(cur_dir);
                } else if second.is_none() {
                    second = Some(cur_dir);
                } else {
                    panic!("Too many exit directions");
                }
            }
            next_dir = cur_dir.next();
        }
        (first.unwrap(), second.unwrap())
    }

    /// Maps each character to its corresponding PipeSection type
    fn read_maze(maze_str: &str) -> Vec<Vec<PipeSection>> {
        maze_str
            .lines()
            .map(|line| line.chars().map(PipeSection::from_char).collect())
            .collect()
    }

    /// Finds the start position in the maze (marked with an `'S'`)
    fn find_start(maze: &[Vec<PipeSection>]) -> (usize, usize) {
        for (row, line) in maze.iter().enumerate() {
            for (col, section) in line.iter().enumerate() {
                if *section == PipeSection::Start {
                    return (row, col);
                }
            }
        }
        panic!("No start found");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeSection {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
    Marker,
    Inside,
    Outside,
}

impl PipeSection {
    /// Returns the PipeSection type that has exits in the given directions
    fn from_exit_directions(dir0: Direction, dir1: Direction) -> PipeSection {
        use Direction::*;
        use PipeSection::*;
        match (dir0, dir1) {
            (North, South) | (South, North) => NS,
            (East, West) | (West, East) => EW,
            (North, East) | (East, North) => NE,
            (North, West) | (West, North) => NW,
            (South, West) | (West, South) => SW,
            (South, East) | (East, South) => SE,
            (_, _) => panic!("Unexpected exit directions: {:?} {:?}", dir0, dir1),
        }
    }

    /// Returns the PipeSection type that corresponds to the given character
    fn from_char(c: char) -> PipeSection {
        match c {
            '|' => PipeSection::NS,
            '-' => PipeSection::EW,
            'L' => PipeSection::NE,
            'J' => PipeSection::NW,
            '7' => PipeSection::SW,
            'F' => PipeSection::SE,
            '.' => PipeSection::Ground,
            'S' => PipeSection::Start,
            'X' => PipeSection::Marker,
            _ => panic!("Unknown pipe section: {}", c),
        }
    }

    /// Returns the direction that the pipe section exits to given the entry direction
    fn exit_direction(&self, entry_direction: Direction) -> Direction {
        use Direction::*;
        use PipeSection::*;
        match (self, entry_direction) {
            (NS, North) => South,
            (NS, South) => North,
            (EW, East) => West,
            (EW, West) => East,
            (NE, North) => East,
            (NE, East) => North,
            (NW, North) => West,
            (NW, West) => North,
            (SW, South) => West,
            (SW, West) => South,
            (SE, South) => East,
            (SE, East) => South,
            (_, _) => panic!(
                "Unexpected pipe section and entry direction: {:?} {:?}",
                self, entry_direction
            ),
        }
    }

    /// Returns true if the pipe section has an entrance from the given direction
    fn has_entrance_from(&self, entry_direction: Direction) -> bool {
        use Direction::*;
        use PipeSection::*;
        match (self, entry_direction) {
            (NS, North) | (NS, South) => true,
            (NE, North) | (NE, East) => true,
            (NW, North) | (NW, West) => true,
            (SW, South) | (SW, West) => true,
            (SE, South) | (SE, East) => true,
            (EW, East) | (EW, West) => true,
            (_, _) => false,
        }
    }

    /// Returns endpoints for each PipeSection type (i.e. NS -> North, South)
    #[allow(dead_code)]
    fn endpoints(&self) -> impl Iterator<Item = Direction> {
        use Direction::*;
        use PipeSection::*;
        match self {
            NS => vec![North, South],
            EW => vec![East, West],
            NE => vec![North, East],
            NW => vec![North, West],
            SW => vec![South, West],
            SE => vec![South, East],
            Ground => vec![],
            Start => vec![],
            Marker | Inside | Outside => panic!("{:?} has no endpoints", self),
        }
        .into_iter()
    }

    fn is_corner(&self) -> bool {
        use PipeSection::*;
        matches!(self, NE | NW | SW | SE)
    }

    fn is_straight(&self) -> bool {
        use PipeSection::*;
        matches!(self, NS | EW)
    }

    fn is_vertical(&self) -> bool {
        use PipeSection::*;
        matches!(self, NS)
    }

    fn is_horizontal(&self) -> bool {
        use PipeSection::*;
        matches!(self, EW)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn next(&self) -> Option<Direction> {
        use Direction::*;
        match self {
            North => Some(East),
            East => Some(South),
            South => Some(West),
            West => None,
        }
    }

    pub fn flip(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

impl AddAssign<Direction> for (usize, usize) {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;
        let (row, col) = self;
        match rhs {
            North => (row - 1, col),
            South => (row + 1, col),
            East => (row, col + 1),
            West => (row, col - 1),
        }
    }
}

impl From<char> for PipeSection {
    fn from(c: char) -> PipeSection {
        PipeSection::from_char(c)
    }
}

impl std::fmt::Display for PipeSection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PipeSection::*;
        let c = if f.alternate() {
            match self {
                SE => '┌',
                NS => '│',
                NE => '└',
                EW => '─',
                NW => '┘',
                SW => '┐',
                Ground => '.',
                Start => 'S',
                Marker => 'X',
                Inside => 'I',
                Outside => 'O',
            }
        } else {
            match self {
                NS => '|',
                EW => '-',
                NE => 'L',
                NW => 'J',
                SW => '7',
                SE => 'F',
                Ground => '.',
                Start => 'S',
                Marker => 'X',
                Inside => 'I',
                Outside => 'O',
            }
        };
        write!(f, "{}", c)
    }
}

fn part1() {
    // let (input, expected_steps) = (include_str!("sample1a.txt"), Some(4));
    let (input, expected_steps) = (include_str!("sample1b.txt"), Some(8));
    // let (input, expected_steps) = (include_str!("my_input.txt"), Some(6697));

    let maze: PipeMaze = add_margin(input).parse().unwrap();

    // Travel directions from both starting positions
    let (mut dir0, mut dir1) = maze.start_exit_directions();
    let mut pos0 = maze.start + dir0;
    let mut pos1 = maze.start + dir1;
    let mut num_steps = 1;

    // Navigate the maze until the two paths meet
    while pos0 != pos1 {
        let pipe0 = maze.pipe_section_at(pos0);
        let pipe1 = maze.pipe_section_at(pos1);
        let next_dir0 = pipe0.exit_direction(dir0.flip());
        let next_dir1 = pipe1.exit_direction(dir1.flip());

        pos0 += next_dir0;
        pos1 += next_dir1;
        dir0 = next_dir0;
        dir1 = next_dir1;
        num_steps += 1;
    }

    println!("Part 1 - Steps: {}", num_steps);

    if let Some(expected_steps) = expected_steps {
        assert_eq!(num_steps, expected_steps);
    }
}

/// Calculates the area of a polygon using the trapezoid method of the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)
fn polygon_area_trapezoid(path: &[(usize, usize)]) -> f64 {
    let signed_area = path
        .windows(2)
        .map(|pair| {
            let (x0, y0) = pair[0];
            let (x1, y1) = pair[1];
            let x0 = x0 as isize;
            let x1 = x1 as isize;
            let y0 = y0 as isize;
            let y1 = y1 as isize;
            (y0 + y1) * (x0 - x1)
        })
        .sum::<isize>()
        / 2;
    signed_area.abs() as f64
}

fn part2_picks_theorum(input: &str, expected_contained_tiles: Option<usize>) {
    let maze: PipeMaze = add_margin(input).parse().unwrap();

    // Travel directions from the starting position, only going one way
    let (mut dir, _) = maze.start_exit_directions();

    // Follow the maze, counting the length of the path
    let mut route = Vec::new();
    route.push(maze.start);
    let mut pos = maze.start + dir;

    // Follow the maze until we return to the start
    while pos != maze.start {
        route.push(pos);
        let pipe = maze.pipe_section_at(pos);
        let next_dir = pipe.exit_direction(dir.flip());
        pos += next_dir;
        dir = next_dir;
    }

    route.push(maze.start); // To complete the loop, need to return to the start

    let route = route;

    // Use Pick's theorem to count the number of tiles inside the polygon.
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // First we need to total area of the polygon formed by the route of the
    // pipes (A). We will use shoelace formula to calculate this.
    let total_area = polygon_area_trapezoid(&route);

    // The number of segments in the pipe route is the number of boundary points (b)
    // we added one extra segment to close the loop, so we subtract one
    let b = route.len() - 1;
    // i = A - (b/2) + 1
    #[allow(non_snake_case)]
    let A = total_area as usize; // we know the area must be an integer because we only have rectangles
    let internal_points_picks = A - (b / 2) + 1;

    println!(
        "Part 2 - Internal points via Pick's Theorem: {}",
        internal_points_picks
    );

    if let Some(expected_tiles) = expected_contained_tiles {
        assert_eq!(internal_points_picks, expected_tiles);
    }
}

/// Count internal tiles by scanning the maze. This also identifies the internal
/// tiles, rather than just counting them like using Pick's theorem.
fn part2_scanlines(input: &str, expected_contained_tiles: Option<usize>) {
    let maze: PipeMaze = add_margin(input).parse().unwrap();

    // Let's mark the path of the pipe we're interested in
    let (mut dir, _) = maze.start_exit_directions();
    let mut loop_marked_maze = maze.clone();
    loop_marked_maze.maze[maze.start.0][maze.start.1] = PipeSection::Marker;
    let mut pos = maze.start + dir;

    // Follow the maze until we return to the start
    while pos != maze.start {
        let pipe = maze.pipe_section_at(pos);
        let next_dir = pipe.exit_direction(dir.flip());
        loop_marked_maze.maze[pos.0][pos.1] = PipeSection::Marker;
        pos += next_dir;
        dir = next_dir;
    }

    // Now that we know where our pipe is, we scan each line of the maze (input file),
    // marking which tiles are contained within our pipe's loop.
    let mut inside_outside_maze = maze.clone();
    let loop_marked_maze = loop_marked_maze;
    let (dir0, dir1) = maze.start_exit_directions();
    let start_tile = PipeSection::from_exit_directions(dir0, dir1);
    let mut num_internal_tiles_scanned = 0;

    // Scan each row
    for (r, row) in maze.maze.iter().enumerate() {
        use PipeSection::*;
        let mut inside = false;
        let mut prev_unmatched_corner = None;

        // Scan west to east in each row
        for (c, tile) in row.iter().enumerate() {
            let tile = if (r, c) == maze.start {
                start_tile
            } else {
                *tile
            };

            if loop_marked_maze.pipe_section_at((r, c)) == Marker {
                if tile.is_vertical() {
                    inside ^= true;
                } else if tile.is_corner() {
                    if let Some(prev_corner) = prev_unmatched_corner {
                        if do_corners_form_u(prev_corner, tile) {
                            inside ^= true;
                        }
                        prev_unmatched_corner = None;
                    } else {
                        inside ^= true;
                        prev_unmatched_corner = Some(tile);
                    }
                }
            } else if inside {
                num_internal_tiles_scanned += 1;
                inside_outside_maze.maze[r][c] = bool_to_pipe_section(inside);
            }
        }
    }

    println!("Scanned map:");

    println!(
        "{}",
        colorize_maze(&maze, &loop_marked_maze, &inside_outside_maze)
    );

    println!(
        "Internal tiles using scanlines: {}",
        num_internal_tiles_scanned
    );

    if let Some(expected_contained_tiles) = expected_contained_tiles {
        assert_eq!(num_internal_tiles_scanned, expected_contained_tiles);
    }
}

/// Colorizes the maze for easier visualization when printing to the console
fn colorize_maze(original: &PipeMaze, marked: &PipeMaze, inside_outside: &PipeMaze) -> String {
    use inline_colorization::*;
    let mut result = String::new();

    for (r, orig_row) in original.maze.iter().enumerate() {
        for (c, tile) in orig_row.iter().enumerate() {
            if (r, c) == original.start {
                result.push_str(color_bright_red);
                result.push_str(&format!("{:#}", tile));
                result.push_str(color_reset);
            } else if marked.pipe_section_at((r, c)) == PipeSection::Marker {
                result.push_str(color_bright_green);
                result.push_str(&format!("{:#}", tile));
                result.push_str(color_reset);
            } else if inside_outside.pipe_section_at((r, c)) == PipeSection::Inside {
                result.push_str(color_bright_magenta);
                result.push_str(bg_cyan);
                // result.push_str(&format!("{:#}", tile));
                result.push('X');
                result.push_str(bg_reset);
                result.push_str(color_reset);
            } else {
                // result.push_str(&format!("{:#}", tile));
                result.push('~');
            }
        }
        result.push('\n');
    }
    result
}

fn bool_to_pipe_section(b: bool) -> PipeSection {
    if b {
        PipeSection::Inside
    } else {
        PipeSection::Outside
    }
}

/// Returns true if the two corners form a U shape
fn do_corners_form_u(left: PipeSection, right: PipeSection) -> bool {
    use PipeSection::*;
    match (left, right) {
        (NE, NW) | (SE, SW) => true,
        (_, _) => false,
    }
}

fn main() {
    part1();

    // let (input, expected_contained_tiles) = (include_str!("sample2a.txt"), Some(4));
    // let (input, expected_contained_tiles) = (include_str!("sample2b.txt"), Some(8_usize));
    let (input, expected_contained_tiles) = (include_str!("sample2c.txt"), Some(10));
    // let (input, expected_contained_tiles) = (include_str!("my_input.txt"), Some(423));

    part2_picks_theorum(input, expected_contained_tiles);
    part2_scanlines(input, expected_contained_tiles);
}
