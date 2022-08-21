use std::fmt;
use svg::node::element::Group;
use svg::Document;

use crate::tile::Tile;

#[derive(Debug)]
pub struct TileGrid {
    rows: usize,
    cols: usize,
    tiles: Vec<Tile>,
}

impl TileGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        TileGrid {
            rows,
            cols,
            tiles: Vec::new(),
        }
    }

    // take the mutable ref to self and return immutable ref to self
    pub fn randomize(&mut self) -> &Self {
        self.randomize_mut()
    }

    // take the mutable ref to self and return mutable ref to self
    pub fn randomize_mut(&mut self) -> &mut Self {
        self.tiles.clear();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let tile = self.gen_tile(i, j);
                self.tiles.push(tile);
            }
        }
        self
    }

    pub fn gen_tile(&self, i: usize, j: usize) -> Tile {
        match (i, j) {
            (0, 0) => Tile::new_random(),
            (0, j) => Tile::new_with_constraints(None, Some(&self.get_tile(0, j - 1)), None, None),
            (i, 0) => Tile::new_with_constraints(Some(&self.get_tile(i - 1, 0)), None, None, None),
            (i, j) => Tile::new_with_constraints(
                Some(&self.get_tile(i - 1, j)),
                Some(&self.get_tile(i, j - 1)),
                None,
                None,
            ),
        }
    }

    pub fn reflect_v(&self) -> Self {
        let mut tile_grid = TileGrid::new(self.rows, self.cols * 2);
        for i in 0..self.rows {
            for j in 0..self.cols {
                tile_grid.tiles.push(self.get_tile(i, j).clone());
            }
            for j in 0..self.cols {
                tile_grid
                    .tiles
                    .push(self.get_tile(i, self.cols - j - 1).reflect_v());
            }
        }

        tile_grid
    }

    pub fn reflect_h(&self) -> Self {
        let mut tile_grid = TileGrid::new(self.rows * 2, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                tile_grid.tiles.push(self.get_tile(i, j).clone());
            }
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                tile_grid
                    .tiles
                    .push(self.get_tile(self.rows - i - 1, j).reflect_h());
            }
        }

        tile_grid
    }

    pub fn get_tile(&self, i: usize, j: usize) -> &Tile {
        &self.tiles[self.cols * i + j]
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.rows {
            let mut row_string = String::new();
            for j in 0..self.cols {
                let tile_as_string = self.get_tile(i, j).to_string();
                row_string.push_str(tile_as_string.split("\n").collect::<Vec<&str>>()[0]);
            }
            row_string.push_str("\n");
            for j in 0..self.cols {
                let tile_as_string = self.get_tile(i, j).to_string();
                row_string.push_str(tile_as_string.split("\n").collect::<Vec<&str>>()[1]);
            }
            row_string.push_str("\n");
            for j in 0..self.cols {
                let tile_as_string = self.get_tile(i, j).to_string();
                row_string.push_str(tile_as_string.split("\n").collect::<Vec<&str>>()[2]);
            }
            row_string.push_str("\n");
            result.push_str(&row_string);
        }

        result
    }

    pub fn to_svg(&self, tile_size: Option<i32>) -> Group {
        let tile_size = tile_size.unwrap_or(30);

        let mut offset_x: i32 = 0;
        let mut offset_y: i32 = 0;
        let mut group = Group::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if j == 0 {
                    offset_x = 0
                }

                let tile_svg =
                    self.get_tile(i, j)
                        .to_svg(Some(offset_x), Some(offset_y), Some(tile_size));
                group = group.add(tile_svg);

                offset_x += tile_size;
            }
            offset_y += tile_size;
        }

        group
    }

    pub fn save_svg(&self, filename: &str, tile_size: Option<i32>) {
        let tile_size = tile_size.unwrap_or(30);
        let tile_grid_svg = self.to_svg(Some(tile_size));
        let document = Document::new()
            .set(
                "viewbox",
                format!(
                    "0 0 {} {}",
                    self.rows * (tile_size as usize),
                    self.cols * (tile_size as usize)
                ),
            )
            .add(tile_grid_svg);

        svg::save(filename, &document).unwrap();
    }
}

impl fmt::Display for TileGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
