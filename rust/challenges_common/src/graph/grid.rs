use itertools::Itertools;

pub struct Grid<N> {
    width: usize,
    content: Vec<N>,
}

impl<N> Grid<N> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.content.len() / self.width
    }

    pub fn at(&self, coord: &Coord) -> Option<&N> {
        if coord.x < self.width {
            self.content.get(coord.x + self.width * coord.y)
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, coord: &Coord) -> Option<&mut N> {
        if coord.x < self.width {
            self.content.get_mut(coord.x + self.width * coord.y)
        } else {
            None
        }
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.width()).flat_map(|x| (0..self.height()).map(move |y| Coord { x, y }))
    }

    pub fn neightbours<'a>(&'a self, coord: &Coord) -> impl Iterator<Item = (Coord, &N)> + 'a {
        coord
            .neighbours(false)
            .filter_map(|coord| self.at(&coord).map(move |n| (coord, n)))
    }
}

impl<IntoIt, N> From<IntoIt> for Grid<N>
where
    IntoIt: IntoIterator,
    IntoIt::Item: IntoIterator<Item = N>,
{
    fn from(into_it: IntoIt) -> Self {
        let mut width = None;

        let content = into_it
            .into_iter()
            .flat_map(|line| match width {
                None => {
                    let line = line.into_iter().collect_vec();
                    width = Some(line.len());
                    line
                }
                Some(_) => line.into_iter().collect_vec(),
            })
            .collect();

        Self {
            width: width.unwrap_or(0),
            content,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn manhattan_dist_to(&self, to: &Coord) -> usize {
        self.x.abs_diff(to.x) + self.y.abs_diff(to.y)
    }

    pub fn neighbours(&self, with_diag: bool) -> impl Iterator<Item = Coord> {
        (-1i8..=1)
            .flat_map(|diff_x| (-1i8..=1).map(move |diff_y| (diff_x, diff_y)))
            .filter(|(diff_x, diff_y)| diff_x != &0 || diff_y != &0)
            .filter(move |(diff_x, diff_y)| with_diag || (diff_x.abs() + diff_y.abs() == 1))
            .map(|(diff_x, diff_y)| (self.x as i32 + diff_x as i32, self.y as i32 + diff_y as i32))
            .filter(|(x, y)| x >= &0 && y >= &0)
            .map(|(x, y)| Coord {
                x: x as usize,
                y: y as usize,
            })
            .collect_vec()
            .into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::Coord;

    #[test]
    fn neighbours_of_11_with_diag_are_8() {
        let coord = Coord { x: 1, y: 1 };
        assert_eq!(coord.neighbours(true).count(), 8)
    }

    #[test]
    fn neighbours_of_11_are_4() {
        let coord = Coord { x: 1, y: 1 };
        assert_eq!(coord.neighbours(false).count(), 4)
    }

    #[test]
    fn neightbours_of_00_are_2() {
        let coord = Coord { x: 0, y: 0 };
        assert_eq!(coord.neighbours(false).count(), 2)
    }
}
