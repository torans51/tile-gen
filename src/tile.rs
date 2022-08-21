use std::fmt;
use svg::node::element::path::Data;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::Document;

enum PosBit {
    TOP,
    LEFT,
    BOTTOM,
    RIGHT,
}

impl PosBit {
    fn value(&self) -> u8 {
        match *self {
            PosBit::TOP => 0b0001 << 0,
            PosBit::LEFT => 0b0001 << 1,
            PosBit::BOTTOM => 0b0001 << 2,
            PosBit::RIGHT => 0b0001 << 3,
        }
    }

    fn bit_on(&self, v: u8) -> bool {
        match *self {
            PosBit::TOP => v >> 0 & 1 == 1,
            PosBit::LEFT => v >> 1 & 1 == 1,
            PosBit::BOTTOM => v >> 2 & 1 == 1,
            PosBit::RIGHT => v >> 3 & 1 == 1,
        }
    }

    // fn bit_off(&self, v: u8) -> bool {
    //     !self.bit_on(v)
    // }
}

#[test]
fn test_value() {
    assert_eq!(PosBit::TOP.value(), 1);
    assert_eq!(PosBit::LEFT.value(), 2);
    assert_eq!(PosBit::BOTTOM.value(), 4);
    assert_eq!(PosBit::RIGHT.value(), 8);
}

#[test]
fn test_bit_on() {
    assert_eq!(PosBit::TOP.bit_on(0), false);
    assert_eq!(PosBit::LEFT.bit_on(0), false);
    assert_eq!(PosBit::BOTTOM.bit_on(0), false);
    assert_eq!(PosBit::RIGHT.bit_on(0), false);

    assert_eq!(PosBit::TOP.bit_on(1), true);
    assert_eq!(PosBit::LEFT.bit_on(2), true);
    assert_eq!(PosBit::BOTTOM.bit_on(4), true);
    assert_eq!(PosBit::RIGHT.bit_on(8), true);

    assert_eq!(PosBit::TOP.bit_on(11), true);
    assert_eq!(PosBit::LEFT.bit_on(11), true);
    assert_eq!(PosBit::BOTTOM.bit_on(11), false);
    assert_eq!(PosBit::RIGHT.bit_on(11), true);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    top: bool,
    left: bool,
    bottom: bool,
    right: bool,
}

impl Tile {
    pub fn new(top: bool, left: bool, bottom: bool, right: bool) -> Self {
        Tile {
            top,
            left,
            bottom,
            right,
        }
    }

    pub fn new_random() -> Self {
        Tile::new(
            rand::random::<bool>(),
            rand::random::<bool>(),
            rand::random::<bool>(),
            rand::random::<bool>(),
        )
    }

    pub fn new_with_constraints(
        top_tile: Option<&Tile>,
        left_tile: Option<&Tile>,
        bottom_tile: Option<&Tile>,
        right_tile: Option<&Tile>,
    ) -> Self {
        let top = top_tile.map_or(rand::random::<bool>(), |v| v.bottom);
        let left = left_tile.map_or(rand::random::<bool>(), |v| v.right);
        let bottom = bottom_tile.map_or(rand::random::<bool>(), |v| v.top);
        let right = right_tile.map_or(rand::random::<bool>(), |v| v.left);
        Tile::new(top, left, bottom, right)
    }

    pub fn from_bits(bits: u8) -> Self {
        assert!(
            bits <= PosBit::TOP.value()
                + PosBit::LEFT.value()
                + PosBit::BOTTOM.value()
                + PosBit::RIGHT.value()
        );

        let top = PosBit::TOP.bit_on(bits);
        let left = PosBit::LEFT.bit_on(bits);
        let bottom = PosBit::BOTTOM.bit_on(bits);
        let right = PosBit::RIGHT.bit_on(bits);
        Tile::new(top, left, bottom, right)
    }

    pub fn transpose(&self) -> Self {
        Tile::new(self.left, self.top, self.right, self.bottom)
    }

    pub fn reflect_v(&self) -> Self {
        Tile::new(self.top, self.right, self.bottom, self.left)
    }

    pub fn reflect_h(&self) -> Self {
        Tile::new(self.bottom, self.left, self.top, self.right)
    }

    pub fn to_bits(&self) -> u8 {
        let mut bits = 0;
        if self.top {
            bits += PosBit::TOP.value();
        }
        if self.left {
            bits += PosBit::LEFT.value();
        }
        if self.bottom {
            bits += PosBit::BOTTOM.value();
        }
        if self.right {
            bits += PosBit::RIGHT.value();
        }
        bits
    }

    pub fn to_string(&self) -> String {
        let tile_as_bit = self.to_bits();
        // 25A1 empty square with border
        // 25A0 small square
        // 25AA very small square
        // 25CC empty small circle with dotted border
        let c = match tile_as_bit {
            0b0000 => {
                "\u{25CC} \u{25CC} \u{25CC} \n\u{25CC} \u{25CC} \u{25CC} \n\u{25CC} \u{25CC} \u{25CC} \n"
            }
            0b0001 => {
                "\u{25AA} \u{25A0} \u{25AA} \n\u{25CC} \u{25AA} \u{25CC} \n\u{25CC} \u{25CC} \u{25CC} \n"
            }
            0b0010 => {
                "\u{25AA} \u{25CC} \u{25CC} \n\u{25A0} \u{25AA} \u{25CC} \n\u{25AA} \u{25CC} \u{25CC} \n"
            }
            0b0011 => {
                "\u{25A0} \u{25A0} \u{25AA} \n\u{25A0} \u{25AA} \u{25CC} \n\u{25AA} \u{25CC} \u{25CC} \n"
            }
            0b0100 => {
                "\u{25CC} \u{25CC} \u{25CC} \n\u{25CC} \u{25AA} \u{25CC} \n\u{25AA} \u{25A0} \u{25AA} \n"
            }
            0b0101 => {
                "\u{25AA} \u{25A0} \u{25AA} \n\u{25CC} \u{25AA} \u{25CC} \n\u{25AA} \u{25A0} \u{25AA} \n"
            }
            0b0110 => {
                "\u{25AA} \u{25CC} \u{25CC} \n\u{25A0} \u{25AA} \u{25CC} \n\u{25A0} \u{25A0} \u{25AA} \n"
            }
            0b0111 => {
                "\u{25A0} \u{25A0} \u{25AA} \n\u{25A0} \u{25AA} \u{25CC} \n\u{25A0} \u{25A0} \u{25AA} \n"
            }
            0b1000 => {
                "\u{25CC} \u{25CC} \u{25AA} \n\u{25CC} \u{25AA} \u{25A0} \n\u{25CC} \u{25CC} \u{25AA} \n"
            }
            0b1001 => {
                "\u{25AA} \u{25A0} \u{25A0} \n\u{25CC} \u{25AA} \u{25A0} \n\u{25CC} \u{25CC} \u{25AA} \n"
            }
            0b1010 => {
                "\u{25AA} \u{25CC} \u{25AA} \n\u{25A0} \u{25AA} \u{25A0} \n\u{25AA} \u{25CC} \u{25AA} \n"
            }
            0b1011 => {
                "\u{25A0} \u{25A0} \u{25A0} \n\u{25A0} \u{25AA} \u{25A0} \n\u{25AA} \u{25CC} \u{25AA} \n"
            }
            0b1100 => {
                "\u{25CC} \u{25CC} \u{25AA} \n\u{25CC} \u{25AA} \u{25A0} \n\u{25AA} \u{25A0} \u{25A0} \n"
            }
            0b1101 => {
                "\u{25AA} \u{25A0} \u{25A0} \n\u{25CC} \u{25AA} \u{25A0} \n\u{25AA} \u{25A0} \u{25A0} \n"
            }
            0b1110 => {
                "\u{25AA} \u{25CC} \u{25AA} \n\u{25A0} \u{25AA} \u{25A0} \n\u{25A0} \u{25A0} \u{25A0} \n"
            }
            0b1111 => {
                "\u{25A0} \u{25A0} \u{25A0} \n\u{25A0} \u{25AA} \u{25A0} \n\u{25A0} \u{25A0} \u{25A0} \n"
            }
            _ => "",
        };
        String::from(c)
    }

    pub fn to_svg(
        &self,
        origin_x: Option<i32>,
        origin_y: Option<i32>,
        tile_size: Option<i32>,
    ) -> Group {
        let origin_x = origin_x.unwrap_or(0);
        let origin_y = origin_y.unwrap_or(0);
        let tile_size = tile_size.unwrap_or(30);

        let half_tile_size = tile_size / 2;
        let fill_color = |on| if on { "red" } else { "white" };

        let tile_as_bit = self.to_bits();
        let data_top = Data::new()
            .move_to((origin_x, origin_y))
            .line_by((half_tile_size, half_tile_size))
            .line_by((half_tile_size, -half_tile_size))
            .close();
        let path_top = Path::new()
            .set("fill", fill_color(PosBit::TOP.bit_on(tile_as_bit)))
            .set("d", data_top);

        let data_left = Data::new()
            .move_to((origin_x, origin_y + tile_size))
            .line_by((half_tile_size, -half_tile_size))
            .line_by((-half_tile_size, -half_tile_size))
            .close();
        let path_left = Path::new()
            .set("fill", fill_color(PosBit::LEFT.bit_on(tile_as_bit)))
            .set("d", data_left);

        let data_bottom = Data::new()
            .move_to((origin_x + tile_size, origin_y + tile_size))
            .line_by((-half_tile_size, -half_tile_size))
            .line_by((-half_tile_size, half_tile_size))
            .close();
        let path_bottom = Path::new()
            .set("fill", fill_color(PosBit::BOTTOM.bit_on(tile_as_bit)))
            .set("d", data_bottom);

        let data_right = Data::new()
            .move_to((origin_x + tile_size, origin_y))
            .line_by((-half_tile_size, half_tile_size))
            .line_by((half_tile_size, half_tile_size))
            .close();
        let path_right = Path::new()
            .set("fill", fill_color(PosBit::RIGHT.bit_on(tile_as_bit)))
            .set("d", data_right);

        Group::new()
            .add(path_top)
            .add(path_left)
            .add(path_bottom)
            .add(path_right)
    }

    pub fn save_svg(&self, filename: &str, tile_size: Option<i32>) {
        let tile_size = tile_size.unwrap_or(30);
        let tile_svg = self.to_svg(Some(0), Some(0), Some(tile_size));
        let document = Document::new()
            .set("viewbox", format!("0 0 {} {}", tile_size, tile_size))
            .add(tile_svg);

        svg::save(filename, &document).unwrap();
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn test_from_bits() {
    assert_eq!(Tile::from_bits(0), Tile::new(false, false, false, false));
    assert_eq!(Tile::from_bits(1), Tile::new(true, false, false, false));
    assert_eq!(Tile::from_bits(2), Tile::new(false, true, false, false));
    assert_eq!(Tile::from_bits(3), Tile::new(true, true, false, false));
    assert_eq!(Tile::from_bits(4), Tile::new(false, false, true, false));
    assert_eq!(Tile::from_bits(5), Tile::new(true, false, true, false));
    assert_eq!(Tile::from_bits(6), Tile::new(false, true, true, false));
    assert_eq!(Tile::from_bits(7), Tile::new(true, true, true, false));
    assert_eq!(Tile::from_bits(8), Tile::new(false, false, false, true));
    assert_eq!(Tile::from_bits(9), Tile::new(true, false, false, true));
    assert_eq!(Tile::from_bits(10), Tile::new(false, true, false, true));
    assert_eq!(Tile::from_bits(11), Tile::new(true, true, false, true));
    assert_eq!(Tile::from_bits(12), Tile::new(false, false, true, true));
    assert_eq!(Tile::from_bits(13), Tile::new(true, false, true, true));
    assert_eq!(Tile::from_bits(14), Tile::new(false, true, true, true));
    assert_eq!(Tile::from_bits(15), Tile::new(true, true, true, true));
}

#[test]
#[should_panic]
fn test_from_bits_panic() {
    Tile::from_bits(16);
}
