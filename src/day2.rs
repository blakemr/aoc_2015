use std::cmp;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let mut f = File::open("inputs/day2.txt").expect("File not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("ERR - read_to_string");

    let input = input.split('\n');

    let mut feet_of_paper = 0;

    for x in input {
        let lwh: Vec<&str> = x.split('x').collect();

        let l = lwh[0].trim().parse::<i32>().expect("error on l");
        let w = lwh[1].trim().parse::<i32>().expect("error on w");
        let h = lwh[2].trim().parse::<i32>().expect("error on h");

        // Calculate the area of each side
        let lw = l * w;
        let wh = w * h;
        let lh = l * h;

        // Add smallest side as extra
        let sm = cmp::min(cmp::min(lw, wh), lh);

        feet_of_paper += 2 * (lw + wh + lh) + sm;
        println!("{}", feet_of_paper);
    }
}
