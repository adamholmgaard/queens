use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};
use crate::state::coordinate::Coordinate;

// A set of contiguous indices on the grid
#[derive(Clone, Debug)]
pub struct Section {
    inner: Range<usize>,
}

impl Into<Section> for usize {
    fn into(self) -> Section {
        Section {
            inner: self..self + 1,
        }
    }
}

impl Into<Section> for Range<usize> {
    fn into(self) -> Section {
        Section { inner: self }
    }
}

impl Into<Section> for RangeInclusive<usize> {
    fn into(self) -> Section {
        Section {
            inner: self.clone().into_inner().0..self.into_inner().1 + 1,
        }
    }
}

fn section<R: Into<Section>>(c: R) -> Section {
    c.into()
}

// Colored area of layout
#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
pub struct Area {
    sections: Vec<usize>,
    color: u8,
}

impl Area {
    fn from_sections(sections: Vec<Section>, col: u8) -> Area {
        let mut res = Vec::new();

        for section in sections {
            for i in section.inner {
                res.push(i);
            }
        }

        Self { sections: res, color: col }
    }

    pub fn get_sections(&self) -> &Vec<usize> {
        &self.sections
    }
    
    pub fn get_color(&self) -> u8 {
        self.color
    }
}

// Layout of multiple areas
#[derive(Clone, Debug)]
pub struct Layout {
    areas: Vec<Area>,
}

impl Layout {
    fn new(areas: Vec<Area>) -> Layout {
        Self { areas }
    }
    
    pub fn get_areas(&self) -> &Vec<Area> {
        &self.areas
    }

    fn from_sections(sections: Vec<Vec<Section>>, n: usize) -> Layout {
        if sections.len() != n {
            panic!("{} sections cannot cover {} queens", sections.len(), n)
        }

        let areas = sections
            .iter().enumerate()
            .map(|(i, secs)| Area::from_sections(secs.clone(), i as u8))
            .collect::<Vec<Area>>();
        
        // todo we do not check for overlap right now

        Self { areas }
    }

    pub fn get_area(&self, index: Coordinate) -> Result<Area, &str> {
        for a in self.areas.iter() {
            if a.sections.contains(&index.get()) {
                return Ok(a.clone());
            }
        }

        Err("No colored area found for index")
    }
}

impl Default for Layout {
    fn default() -> Self {
        easy_layout()
    }
}

fn section_to_int(args: Vec<Section>) -> Vec<usize> {
    let mut vec = Vec::new();

    for section in args {
        vec.extend(section.inner.clone());
    }

    vec
}

// Easily solvable layout with n=10
pub fn easy_layout() -> Layout {
    let n = 10;
    let mut res: Vec<Vec<Section>> = vec![];

    for i in 0..n {
        res.push(vec![section(i * n..(i + 1) * n)])
    }

    Layout::from_sections(res, n)
}

// Complex layout with n=10
pub fn complex_layout() -> Layout {
    let n = 10;

    let r1 = vec![
        section(0..n),
        section(n * 2 - 1),
        section(n * 3 - 1),
        section(n * 4 - 1),
        section(n * 5 - 1),
        section(n * 6 - 1),
        section(n * 7 - 1),
        section(n * 8 - 1),
        section(n * 9 - 1),
        section(n * n - 1),
    ];

    let r2 = vec![
        section(n..=n + 2),
        section(n * 2..=n * 2 + 2),
        section(n * 3..=n * 3 + 4),
    ];

    let r3 = vec![
        section(n * 3 - 3),
        section(n * 4 - 3),
        section(n * 5 - 3..n * 5 - 1),
        section(n * 6 - 6..n * 6 - 1),
        section(n * 7 - 5..n * 7 - 1),
        section(n * 8 - 2),
    ];

    let r4 = vec![
        section(n + 3..2 * n - 1),
        section(2 * n + 3..3 * n - 3),
        section(3 * n - 2),
        section(4 * n - 5),
        section(4 * n - 2),
    ];

    let r5 = vec![section(n * 4 + 3..=n * 4 + 4)];

    let r6 = vec![section(n * 4 - 4), section(n * 4 + 5..=n * 4 + 6)];

    let r7 = vec![
        section(n * 4..n * 4 + 3),
        section(n * 5..n * 5 + 4),
        section(n * 6..n * 6 + 4),
        section(n * 7..n * 7 + 2),
        section(n * 8..n * 8 + 5),
        section(n * 9..n * 9 + 5),
    ];

    let r8 = vec![section(n * 7 + 2..=n * 7 + 3)];

    let r9 = vec![
        section(6 * n + 4),
        section(7 * n + 4..7 * n + 8),
        section(8 * n + 5..8 * n + 9),
        section(9 * n + 5..9 * n + 8),
    ];

    let r10 = vec![section(n * n - 2)];

    Layout::from_sections(vec![r1, r2, r3, r4, r5, r6, r7, r8, r9, r10], n)
}
