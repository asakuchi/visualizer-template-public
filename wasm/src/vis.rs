#![allow(non_snake_case, unused_macros)]
// use itertools::Itertools;
// use proconio::input;
// use rand::prelude::*;
// use std::{collections::VecDeque, usize};
use svg::node::element::{Rectangle, Style};
use web_sys::console::log_1;

use super::tools;

const VIEW_SIZE: usize = 100;

pub fn vis(input: &tools::Input, output: &tools::Output, _turn: usize) -> (i64, String, String) {
    log_1(&"start vis".into());

    // scoreはこの関数では計算しない
    let score = 999999999;

    let mut a = input.a.clone();
    let mut p1 = (output.start.0, output.start.1);
    let mut p2 = (output.start.2, output.start.3);

    for &(do_swap, dir1, dir2) in &output.out {
        if do_swap {
            let tmp = a[p1.0][p1.1];
            a[p1.0][p1.1] = a[p2.0][p2.1];
            a[p2.0][p2.1] = tmp;
        }
        if dir1 != !0 {
            if !tools::can_move(input.n, &input.hs, &input.vs, p1.0, p1.1, dir1) {
                return (
                    0,
                    format!("Invalid move: {}", tools::DIRS[dir1]),
                    // (a, p1, p2),
                    "".to_string(),
                );
            }
            p1.0 += tools::DIJ[dir1].0;
            p1.1 += tools::DIJ[dir1].1;
        }
        if dir2 != !0 {
            if !tools::can_move(input.n, &input.hs, &input.vs, p2.0, p2.1, dir2) {
                return (
                    0,
                    format!("Invalid move: {}", tools::DIRS[dir2]),
                    // (a, p1, p2),
                    "".to_string(),
                );
            }
            p2.0 += tools::DIJ[dir2].0;
            p2.1 += tools::DIJ[dir2].1;
        }
    }

    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (0, 0, VIEW_SIZE, VIEW_SIZE))
        .set("width", 800)
        .set("height", 800)
        .set("style", "background-color:white; border: solid");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
    )));

    use svg::node::element::path::Data;
    use svg::node::element::Path;

    let cell_size = 10;

    for y in 0..input.n {
        for x in 0..input.n {
            let text = format!(
                "rgba(255,0,0,{})",
                a[y][x] as f64 / (input.n as f64 * input.n as f64)
            );

            doc = doc.add(
                rect(
                    x * cell_size,
                    y * cell_size,
                    cell_size,
                    cell_size,
                    if (y, x) == p1 {
                        "blue"
                    } else if (y, x) == p2 {
                        "green"
                    } else {
                        &text
                    },
                )
                .set("stroke", "#dddddd")
                .set("stroke-width", 1)
                .set("class", "box"),
            );
        }
    }

    for y in 0..input.n {
        for x in 0..input.n {
            if y != input.n - 1 && x != input.n - 1 && input.vs[y][x] == '1' {
                log_1(&format!("y {y} vs {}", input.vs[y][x]).into());

                let data = Data::new()
                    .move_to((x * cell_size + cell_size, y * cell_size))
                    .line_by((0, cell_size))
                    .close();

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("d", data);

                doc = doc.add(path);
            }

            if y != input.n - 1 && x != input.n - 1 && input.hs[y][x] == '1' {
                log_1(&format!("y {y} vs {}", input.hs[y][x]).into());

                let data = Data::new()
                    .move_to((x * cell_size, y * cell_size + cell_size))
                    .line_by((cell_size, 0))
                    .close();

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("d", data);

                doc = doc.add(path);
            }
        }
    }

    (score as i64, "".to_string(), doc.to_string())
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}
