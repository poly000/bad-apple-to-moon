#![feature(try_blocks)]

use image::{GenericImageView, open};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::error::Error;
use itertools::Itertools;

const VIDEO_WIDTH: usize = 180;
const FILENAME_WTDTH: usize = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let pool = threadpool::ThreadPool::new(4 * num_cpus::get());

    for number in 1..=6574 {
        pool.execute(move|| {
            let (output, input) = (format!("{number:0>width$}.txt", number = number, width = FILENAME_WTDTH), format!("{number:0>width$}.png", number = number, width = FILENAME_WTDTH));
            let (output, input) = (File::create(output).unwrap(), open(input).unwrap());
            let rows = input.pixels().chunks(VIDEO_WIDTH);
            let mut buf = BufWriter::new(output);
            for row in &rows {
                for color in row.map(|pixel| pixel.2) {
                    if color[0] <= 10 {
                        write!(buf, r#"🌚"#).unwrap();
                    } else {
                        write!(buf, r#"🌝"#).unwrap();
                    }
                }
                write!(buf, "\n").unwrap();
            }
        });
    }

    pool.join();

    Ok(())
}
