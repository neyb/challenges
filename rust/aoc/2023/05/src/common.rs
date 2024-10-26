use itertools::Itertools;
use std::ops::Range;
use std::str::FromStr;

pub type Position = u64;
pub type PositionTransformation = i64;

pub struct Mapping {
    transformations: Vec<Transformation>,
}

impl Mapping {
    pub fn map(&self, value: Position) -> Position {
        match self
            .transformations
            .iter()
            .find_or_first(|transformation| transformation.concerns(value))
        {
            Some(transformation) => transformation.map(value),
            None => value,
        }
    }
}

impl<S> TryFrom<Vec<S>> for Mapping
where
    S: AsRef<str>,
{
    type Error = anyhow::Error;

    fn try_from(lines: Vec<S>) -> anyhow::Result<Self> {
        let transformations = lines
            .iter()
            .skip(1)
            .map(|line| line.as_ref().parse::<Transformation>())
            .try_collect()?;

        Ok(Self { transformations })
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let transformations = s.lines().skip(1).map(|line| line.parse()).try_collect()?;
        Ok(Self { transformations })
    }
}

#[derive(Debug, PartialEq)]
pub struct Transformation {
    source_range: Range<Position>,
    transformation: PositionTransformation,
}

impl Transformation {
    fn new(source_range: Range<Position>, transformation: PositionTransformation) -> Self {
        Self {
            source_range,
            transformation,
        }
    }

    fn concerns(&self, value: Position) -> bool {
        self.source_range.contains(&value)
    }

    fn map(&self, value: Position) -> Position {
        if self.source_range.contains(&value) {
            (value as PositionTransformation + self.transformation) as Position
        } else {
            value
        }
    }
}

impl FromStr for Transformation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let elements = s.split(' ').collect_vec();
        let [destination_start, source_start, source_range] = elements.as_slice() else {
            anyhow::bail!("cannot parse transformation {}", s)
        };

        let source_start: u64 = source_start.parse()?;
        let source_range: u64 = source_range.parse()?;
        let destination_start: u64 = destination_start.parse()?;

        Ok(Self::new(
            source_start..source_start + source_range,
            destination_start as PositionTransformation - source_start as PositionTransformation,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_transformation() {
        let transformation = "50 98 2";
        let transformation = Transformation::from_str(transformation).unwrap();
        assert_eq!(transformation, Transformation::new(98..100, -48));
    }
}
