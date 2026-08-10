use crate::buffer::Buffer;

fn plot_line_low(b: &mut Buffer, (x0, y0): (u32, u32), (x1, y1): (u32, u32), color: u32) -> () {
    let dx = x1 as i32 - x0 as i32;
    let mut dy = y1 as i32 - y0 as i32;
    let mut yi = 1_i32;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = y0 as i32;

    for x in x0..x1 {
        b.fill_pixel((x as i32, y), color);
        if d > 0 {
            y = y + yi;
            d = d + (2 * (dy - dx))
        } else {
            d = d + 2 * dy
        }
    }
}

fn plot_line_high(b: &mut Buffer, (x0, y0): (u32, u32), (x1, y1): (u32, u32), color: u32) -> () {
    let mut dx = x1 as i32 - x0 as i32;
    let dy = y1 as i32 - y0 as i32;
    let mut xi = 1_i32;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = (2 * dx) - dy;
    let mut x = x0 as i32;

    for y in y0..y1 {
        b.fill_pixel((x, y as i32), color);
        if d > 0 {
            x = x + xi;
            d = d + (2 * (dx - dy))
        } else {
            d = d + 2 * dx
        }
    }
}

pub fn plot_line(b: &mut Buffer, (x0, y0): (u32, u32), (x1, y1): (u32, u32), color: u32) -> () {
    if (y1 as i32 - y0 as i32).abs() < (x1 as i32 - x0 as i32).abs() {
        if x0 > x1 {
            plot_line_low(b, (x1, y1), (x0, y0), color)
        } else {
            plot_line_low(b, (x0, y0), (x1, y1), color)
        }
    } else {
        if y0 > y1 {
            plot_line_high(b, (x1, y1), (x0, y0), color)
        } else {
            plot_line_high(b, (x0, y0), (x1, y1), color)
        }
    }
}
