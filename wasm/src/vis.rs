#![allow(non_snake_case, unused_macros)]
// use itertools::Itertools;
// use proconio::input;
// use rand::prelude::*;
// use std::{collections::VecDeque, usize};
use svg::node::{
    element::{Rectangle, Style},
    Text,
};
use web_sys::console::log_1;

use crate::tools::{Action, DIJ};

use super::tools;

const VIEW_SIZE: usize = 1_000;

pub fn vis(input: &tools::Input, output: &tools::Output, turn: usize) -> (i64, String, String) {
    log_1(&"start vis".into());

    // scoreはこの関数では計算しない
    let score = 999999999;

    let mut cs = input.cs.clone();

    let mut pos = (0, 0);

    for i in 0..input.N {
        for j in 0..input.N {
            if cs[i][j] == 'A' {
                pos = (i, j);
            }
        }
    }

    let out = output.out.clone();

    for t in 0..turn {
        match out[t] {
            Action::Move(d) => {
                let (di, dj) = DIJ[d];
                pos.0 += di;
                pos.1 += dj;
                if pos.0 >= input.N || pos.1 >= input.N {
                    return (0, format!("Out of the board (turn {t})"), "".to_string());
                }
            }
            Action::Carry(d) => {
                let (di, dj) = DIJ[d];
                if (cs[pos.0][pos.1] < 'a' || cs[pos.0][pos.1] > 'z') && cs[pos.0][pos.1] != '@' {
                    return (0, format!("No item to carry (turn {t})"), "".to_string());
                }
                let c = cs[pos.0][pos.1];
                cs[pos.0][pos.1] = '.';
                pos.0 += di;
                pos.1 += dj;
                if pos.0 >= input.N || pos.1 >= input.N {
                    return (0, format!("Out of the board (turn {t})"), "".to_string());
                }
                if matches!(cs[pos.0][pos.1], '@' | 'a'..='z') {
                    return (0, format!("Collision (turn {t})"), "".to_string());
                } else if matches!(cs[pos.0][pos.1], 'A'..='Z') {
                    if cs[pos.0][pos.1].to_ascii_lowercase() == c {
                        // A += 1;
                    }
                } else {
                    assert_eq!(cs[pos.0][pos.1], '.');
                    cs[pos.0][pos.1] = c;
                }
            }
            Action::Roll(d) => {
                let (di, dj) = DIJ[d];
                if (cs[pos.0][pos.1] < 'a' || cs[pos.0][pos.1] > 'z') && cs[pos.0][pos.1] != '@' {
                    return (0, format!("No item to roll (turn {t})"), "".to_string());
                }
                let c = cs[pos.0][pos.1];
                cs[pos.0][pos.1] = '.';
                let mut crt = pos;
                loop {
                    let next = (crt.0 + di, crt.1 + dj);
                    if next.0 >= input.N
                        || next.1 >= input.N
                        || matches!(cs[next.0][next.1], '@' | 'a'..='z')
                    {
                        cs[crt.0][crt.1] = c;
                        break;
                    } else if matches!(cs[next.0][next.1], 'A'..='Z') {
                        if cs[next.0][next.1].to_ascii_lowercase() == c {
                            // A += 1;
                        }
                        break;
                    } else {
                        crt = next;
                    }
                }
            }
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
    use svg::node::element::Text;

    let cell_size = VIEW_SIZE / input.N;
    let stroke_size = VIEW_SIZE / input.N / 10;

    // for y in 0..input.N {
    //     for x in 0..input.N {
    //         // let text = format!("rgba(255,0,0,{})", cs[y][x]);

    //         doc = doc.add(
    //             rect(x * cell_size, y * cell_size, cell_size, cell_size, "white")
    //                 .set("stroke", "black")
    //                 .set("stroke-width", stroke_size)
    //                 .set("class", "box"), // .set("data-num", a[y][x])
    //         );
    //     }
    // }

    for y in 0..input.N {
        for x in 0..input.N {
            // let text = format!("rgba(255,0,0,{})", cs[y][x]);

            // if (y, x) == pos {
            //     doc = doc.add(
            //         rect(x * cell_size, y * cell_size, cell_size, cell_size, "white")
            //             .set("stroke", "orange")
            //             .set("stroke-width", stroke_size)
            //             .set("class", "box"), // .set("data-num", a[y][x])
            //     );
            // }

            doc = doc.add(
                rect(
                    x * cell_size,
                    y * cell_size,
                    cell_size,
                    cell_size,
                    if cs[y][x] == 'A' {
                        "red"
                    } else if cs[y][x] == 'B' {
                        "green"
                    } else if cs[y][x] == 'C' {
                        "blue"
                    } else if cs[y][x] == 'a' {
                        "#FFDDDD"
                    } else if cs[y][x] == 'b' {
                        "#DDFFDD"
                    } else if cs[y][x] == 'c' {
                        "#DDDDFF"
                    } else {
                        "white"
                    },
                )
                .set("stroke", if (y, x) == pos { "orange" } else { "black" })
                .set("stroke-width", stroke_size)
                .set("class", "box"), // .set("data-num", a[y][x])
            );

            if cs[y][x] != '.' {
                let text = Text::new()
                    .set("x", x * cell_size + 20) // 中央付近
                    .set("y", y * cell_size + 20)
                    .set("font-size", 30)
                    .set("text-anchor", "middle") // 文字を中央揃えに
                    .set("fill", "black") // 文字の色
                    .add(svg::node::Text::new(cs[y][x]));

                doc = doc.add(text);
            }
        }
    }

    // 現在地を同じ内容でもう一度
    for y in 0..input.N {
        for x in 0..input.N {
            if (y, x) == pos {
                doc = doc.add(
                    rect(
                        x * cell_size,
                        y * cell_size,
                        cell_size,
                        cell_size,
                        if cs[y][x] == 'A' {
                            "red"
                        } else if cs[y][x] == 'B' {
                            "green"
                        } else if cs[y][x] == 'C' {
                            "blue"
                        } else if cs[y][x] == 'a' {
                            "#FFDDDD"
                        } else if cs[y][x] == 'b' {
                            "#DDFFDD"
                        } else if cs[y][x] == 'c' {
                            "#DDDDFF"
                        } else {
                            "white"
                        },
                    )
                    .set("stroke", if (y, x) == pos { "orange" } else { "black" })
                    .set("stroke-width", stroke_size)
                    .set("class", "box"), // .set("data-num", a[y][x])
                );

                if cs[y][x] != '.' {
                    let text = Text::new()
                        .set("x", x * cell_size + 20) // 中央付近
                        .set("y", y * cell_size + 20)
                        .set("font-size", 30)
                        .set("text-anchor", "middle") // 文字を中央揃えに
                        .set("fill", "black") // 文字の色
                        .add(svg::node::Text::new(cs[y][x]));

                    doc = doc.add(text);
                }
            }
        }
    }

    // for y in 0..input.n {
    //     for x in 0..input.n {
    //         if x != input.n - 1 && input.vs[y][x] == '1' {
    //             let data = Data::new()
    //                 .move_to((x * cell_size + cell_size, y * cell_size))
    //                 .line_by((0, cell_size))
    //                 .close();

    //             let path = Path::new()
    //                 .set("fill", "none")
    //                 .set("stroke", "black")
    //                 .set("stroke-width", stroke_size)
    //                 .set("d", data);

    //             doc = doc.add(path);
    //         }

    //         if y != input.n - 1 && input.hs[y][x] == '1' {
    //             let data = Data::new()
    //                 .move_to((x * cell_size, y * cell_size + cell_size))
    //                 .line_by((cell_size, 0))
    //                 .close();

    //             let path = Path::new()
    //                 .set("fill", "none")
    //                 .set("stroke", "black")
    //                 .set("stroke-width", stroke_size)
    //                 .set("d", data);

    //             doc = doc.add(path);
    //         }
    //     }
    // }

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
