use std::fmt::{self, Display};
use std::ops::{Add, AddAssign, Index, IndexMut};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_str(s: &str) -> Point {
        let mut it = s.split(',');
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        Point::new(x, y)
    }

    pub fn offset(self, dx: isize, dy: isize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Dir8 {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Dir8 {
    pub const ALL: [Dir8; 8] = [
        Dir8::Up,
        Dir8::Down,
        Dir8::Left,
        Dir8::Right,
        Dir8::UpLeft,
        Dir8::UpRight,
        Dir8::DownLeft,
        Dir8::DownRight,
    ];

    pub fn delta(self) -> (isize, isize) {
        match self {
            Dir8::Up => (0, -1),
            Dir8::Down => (0, 1),
            Dir8::Left => (-1, 0),
            Dir8::Right => (1, 0),
            Dir8::UpLeft => (-1, -1),
            Dir8::UpRight => (1, -1),
            Dir8::DownLeft => (-1, 1),
            Dir8::DownRight => (1, 1),
        }
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Point;
    fn add(self, rhs: (isize, isize)) -> Point {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl AddAssign<(isize, isize)> for Point {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Point {
        let (dx, dy) = rhs.delta();
        Point::new(self.x + dx, self.y + dy)
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        let (dx, dy) = rhs.delta();
        self.x += dx;
        self.y += dy;
    }
}

impl Add<Dir8> for Point {
    type Output = Point;
    fn add(self, rhs: Dir8) -> Point {
        let (dx, dy) = rhs.delta();
        Point::new(self.x + dx, self.y + dy)
    }
}

impl AddAssign<Dir8> for Point {
    fn add_assign(&mut self, rhs: Dir8) {
        let (dx, dy) = rhs.delta();
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone + Display> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            width,
            height,
            data: vec![default; width * height],
        }
    }
}

impl<T: Display> Grid<T> {
    #[inline]
    fn index(&self, p: Point) -> usize {
        (p.y as usize) * self.width + (p.x as usize)
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && (p.x as usize) < self.width && (p.y as usize) < self.height
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        self.in_bounds(p).then(|| &self.data[self.index(p)])
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if self.in_bounds(p) {
            let idx = self.index(p);
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    pub fn set(&mut self, p: Point, value: T) {
        if self.in_bounds(p) {
            let idx = self.index(p);
            self.data[idx] = value;
        }
    }

    pub fn coords(&self) -> impl Iterator<Item = Point> {
        let w = self.width as isize;
        let h = self.height as isize;
        (0..h).flat_map(move |y| (0..w).map(move |x| Point::new(x, y)))
    }

    pub fn neighbors4(&self, p: Point) -> impl Iterator<Item = Point> + '_ {
        Direction::ALL.into_iter().filter_map(move |d| {
            let q = p + d;
            self.in_bounds(q).then_some(q)
        })
    }

    pub fn neighbors8(&self, p: Point) -> impl Iterator<Item = Point> + '_ {
        Dir8::ALL.into_iter().filter_map(move |d| {
            let q = p + d;
            self.in_bounds(q).then_some(q)
        })
    }

    pub fn wrap(&self, p: Point) -> Point {
        let w = self.width as isize;
        let h = self.height as isize;
        Point {
            x: (p.x % w + w) % w,
            y: (p.y % h + h) % h,
        }
    }

    pub fn row(&self, y: isize) -> Option<impl Iterator<Item = &T>> {
        if y >= 0 && (y as usize) < self.height {
            let start = (y as usize) * self.width;
            Some(self.data[start..start + self.width].iter())
        } else {
            None
        }
    }

    pub fn col(&self, x: isize) -> Option<impl Iterator<Item = &T>> {
        if x >= 0 && (x as usize) < self.width {
            Some((0..self.height).map(move |y| &self.data[y * self.width + (x as usize)]))
        } else {
            None
        }
    }
}

impl<T: Display> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, p: Point) -> &Self::Output {
        self.get(p).expect("Grid index out of bounds")
    }
}

impl<T: Display> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        self.get_mut(p).expect("Grid index out of bounds")
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x as isize, y as isize);
                write!(f, "{}", self.data[self.index(p)])?;
            }
            if y + 1 < self.height {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
