use std::io;

fn main() {
    let mut y = 2;
    y = draw_regions(y).unwrap();
    println!("{}", y);
}

fn write(y: usize) -> io::Result<usize> {
    Ok(y + 1)
}

fn write_right(y: usize) -> io::Result<usize> {
    let z = write(y)?;
    println!("write_right: {}", z);
    Ok(z)
}

fn draw_regions(y: usize) -> io::Result<usize> {
    let mut y = y;
    for i in 0..2 {
        y = write_right(y)?;
        println!("draw_regions 1: {}", y);

        y = write_right(y)?;
        println!("draw_regions 2: {}", y);
    }
    Ok(y)
}
