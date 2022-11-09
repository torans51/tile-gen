pub mod tile;
pub mod tile_grid;

use clap::{Parser, ValueEnum};
use tile_grid::TileGrid;

#[derive(Clone, ValueEnum, Debug)]
enum OutputFormat {
    SVG,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = false)]
    debug: bool,

    #[clap(long, value_parser, default_value_t = 10)]
    rows: usize,

    #[clap(long, value_parser, default_value_t = 10)]
    cols: usize,

    #[clap(long, value_parser, default_value_t = true)]
    symmetric: bool,

    #[clap(long, arg_enum, value_parser)]
    output_format: Option<OutputFormat>,

    #[clap(long, value_parser, default_value = "tile-grid")]
    output_filename: String,
}

fn main() {
    let args = Args::parse();

    let mut tile_grid = TileGrid::new(args.rows, args.cols);
    tile_grid.randomize();

    if args.symmetric {
        tile_grid = tile_grid.reflect_v();
        tile_grid = tile_grid.reflect_h();
    }

    match args.output_format {
        Some(OutputFormat::SVG) => {
            let filepath = String::from(format!("{}.svg", args.output_filename));
            tile_grid.save_svg(&filepath, None);
        }
        None => println!("{}", tile_grid),
    }

    // println!("{}", TileGrid::new(10, 10).randomize());
    // println!(
    //     "{}",
    //     TileGrid::new(5, 5).randomize().reflect_v().reflect_h()
    // );
    //
    // println!("{}", Tile::from_bits(11));
    // println!("{}", Tile::from_bits(11).transpose());
    // Tile::new_random().save_svg("tile.svg", None);
    // TileGrid::new(5, 5)
    //     .randomize()
    //     .reflect_v()
    //     .reflect_h()
    //     .save_svg("tile_grid.svg", None);

    // Test
    // let mut t = tile_grid::TileGrid::new(1, 2);
    // t.randomize_mut(); // allowed
    // t.randomize_mut().randomize_mut(); // allowed
    // t.randomize(); // allowed
    // t.randomize(); // allowed
    // t.randomize().randomize(); // not allowed
}
