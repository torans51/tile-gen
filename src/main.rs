pub mod tile;
pub mod tile_grid;

use tile::Tile;
use tile_grid::TileGrid;

fn main() {
    println!("{}", TileGrid::new(10, 10).randomize());
    println!(
        "{}",
        TileGrid::new(5, 5).randomize().reflect_v().reflect_h()
    );

    println!("{}", Tile::from_bits(11));
    println!("{}", Tile::from_bits(11).transpose());
    Tile::new_random().save_svg("tile.svg", None);
    TileGrid::new(5, 5)
        .randomize()
        .reflect_v()
        .reflect_h()
        .save_svg("tile_grid.svg", None);

    // Test
    // let mut t = tile_grid::TileGrid::new(1, 2);
    // t.randomize_mut(); // allowed
    // t.randomize_mut().randomize_mut(); // allowed
    // t.randomize(); // allowed
    // t.randomize(); // allowed
    // t.randomize().randomize(); // not allowed
}

