use std::num::Wrapping;

pub fn is_square(x: u64) -> bool {
    let xtz = x.trailing_zeros();
    if x == 0 {
        return true;
    } else if xtz & 1 != 0 {
        return false;
    }
    let x = Wrapping(x >> xtz);

    let mut y = Wrapping(1_u64);
    for _ in 0..5 {
        y = ((Wrapping(3_u64) - y * y * x) * y) >> 1;
    }

    let mut xrt = x * y;
    if xrt.0 & (1 << 32) != 0 {
        xrt = -xrt;
    }
    let xrt = xrt.0 & ((1 << 32) - 1);

    xrt * xrt == x.0
}

pub fn is_square_by_sqrt(x: u64) -> bool {
    let xrt = (x as f64).sqrt().round() as u64;
    xrt * xrt == x
}
