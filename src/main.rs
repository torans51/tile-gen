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
}
