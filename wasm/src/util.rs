#![allow(non_snake_case, unused_macros)]
use itertools::Itertools;
use proconio::input;
use rand::prelude::*;
use std::{collections::VecDeque, usize};
use svg::node::element::{Rectangle, Style};
use web_sys::console::log_1;

/// キャンディの個数(100固定)
const N: usize = 100;
/// 箱の高さ(10固定)
const W: usize = 10;
/// 箱の幅(10固定)
const H: usize = 10;

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    /// t番目に受け取るキャンディーの味
    pub f: Vec<usize>,
    /// p_t番目の空きマスにt個目のキャンディーが入れられる
    pub p: Vec<usize>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.f.iter().join(" ");

        writeln!(f, "{}", &text)?;

        for i in 0..N {
            writeln!(f, "{}", self.p[i])?;
        }

        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);

    let n = 100;

    input! {
        from f,
        candy: [usize; n],
        p: [usize; n],
    }

    Input { f: candy, p }
}

pub struct Output {
    /// どの方向へ傾けるか
    pub out: Vec<String>,
}

pub fn parse_output(f: &str) -> Output {
    let f = proconio::source::once::OnceSource::from(f);

    input! {
        from f,
        out: [String; 100],
    }

    log_1(&format!("out: {:?}", out).into());

    Output { out }
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);

    let f = (0..N).map(|_| rng.gen_range(1, 4)).collect::<Vec<_>>();

    let p = (0..N)
        .map(|t| rng.gen_range(1, 101 - t))
        .collect::<Vec<_>>();

    Input { f, p }
}

fn calculate_score(input: &Input, out: &Vec<String>) -> (usize, Vec<Vec<usize>>) {
    log_1(&"start calculate_score".into());

    let mut state = vec![vec![0; W]; H];

    // log_1(&format!("state: {:?}", state).into());

    // log_1(&format!("input.p: {:?}", input.p).into());
    // log_1(&format!("out.len(): {}", out.len()).into());

    for turn in 0..out.len() {
        // log_1(&format!("turn: {}", turn).into());

        state = put_candy(10, turn, &input.f, state, input.p[turn]);

        // log_1(&format!("state(put_candy): {:?}", state).into());

        if turn == out.len() - 1 {
            // 最後のslideはしない
            break;
        }

        state = slide(10, state, out[turn].as_str());

        // log_1(&format!("state(slide): {:?}", state).into());
    }

    let mut candy_counter = vec![0; 4];

    for &candy in input.f.iter() {
        candy_counter[candy] += 1;
    }

    // let score = score(10, &state);
    // return (score, state);

    let score = score(10, &state) as f64
        / (candy_counter.into_iter().map(|x| x * x).sum::<usize>() as f64)
        * 1_000_000.;
    return (score.round() as usize, state);
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    log_1(&"start vis".into());

    let (score, state) =
        calculate_score(input, &output.out[0..turn].into_iter().cloned().collect());

    log_1(&format!("score: {}", score).into());
    log_1(&format!("state: {:?}", state).into());

    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (0, 0, W, H))
        .set("width", 800)
        .set("height", 800)
        .set("style", "background-color:white; border: solid");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
    )));

    for y in 0..H {
        for x in 0..W {
            doc = doc.add(
                rect(
                    x,
                    y,
                    1,
                    1,
                    match state[y][x] {
                        1 => "tomato",
                        2 => "green",
                        3 => "orange",
                        _ => "white",
                    },
                )
                // .set("stroke", "black")
                // .set("stroke-width", 1)
                .set("class", "box"),
            );
        }
    }

    (score as i64, "".to_string(), doc.to_string())
}

fn score(n: usize, table: &Vec<Vec<usize>>) -> usize {
    let mut score = 0;

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut visited = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            if table[i][j] == 0 || visited[i][j] {
                continue;
            }

            let candy = table[i][j];

            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            visited[i][j] = true;

            let mut count = 0;

            while let Some(current) = queue.pop_front() {
                count += 1;

                for pattern in &patterns {
                    let next = (
                        current.0 as isize + pattern.0,
                        current.1 as isize + pattern.1,
                    );

                    if 0 > next.0 || next.0 >= n as isize || 0 > next.1 || next.1 >= n as isize {
                        continue;
                    }

                    let next = (next.0 as usize, next.1 as usize);

                    if visited[next.0][next.1] || table[next.0][next.1] != candy {
                        continue;
                    }

                    // 次へ
                    visited[next.0][next.1] = true;
                    queue.push_back(next);
                }
            }

            score += count * count;
        }
    }

    score
}

fn slide(n: usize, mut table: Vec<Vec<usize>>, pattern: &str) -> Vec<Vec<usize>> {
    // let mut table = table.clone();

    match pattern {
        "F" => {
            // 上に寄せる
            for j in 0..n {
                let mut dest = 0;

                for i in dest..n {
                    if table[i][j] != 0 {
                        let tmp = table[dest][j];
                        table[dest][j] = table[i][j];
                        table[i][j] = tmp;

                        dest += 1;
                    }
                }
            }
        }
        "B" => {
            // 下に寄せる
            for j in 0..n {
                let mut dest = n - 1;

                for i in (0..=dest).rev() {
                    if table[i][j] != 0 {
                        let tmp = table[dest][j];
                        table[dest][j] = table[i][j];
                        table[i][j] = tmp;

                        dest -= 1;
                    }
                }
            }
        }
        "L" => {
            // 左に寄せる
            for i in 0..n {
                let mut dest = 0;

                for j in dest..n {
                    if table[i][j] != 0 {
                        let tmp = table[i][dest];
                        table[i][dest] = table[i][j];
                        table[i][j] = tmp;

                        dest += 1;
                    }
                }
            }
        }
        _ => {
            // 右に寄せる
            for i in 0..n {
                let mut dest = n - 1;

                for j in (0..=dest).rev() {
                    if table[i][j] != 0 {
                        let tmp = table[i][dest];
                        table[i][dest] = table[i][j];
                        table[i][j] = tmp;

                        dest -= 1;
                    }
                }
            }
        }
    }

    table
}

fn put_candy(
    n: usize,
    turn: usize,
    f: &Vec<usize>,
    mut table: Vec<Vec<usize>>,
    target: usize,
) -> Vec<Vec<usize>> {
    let mut target = target;
    // let mut table = table.clone();

    'search: for i in 0..n {
        for j in 0..n {
            if table[i][j] != 0 {
                continue;
            }

            target -= 1;

            if target == 0 {
                // println!("update i {} j {} {} -> {}", i, j, table[i][j], f[turn]);
                table[i][j] = f[turn];
                break 'search;
            }
        }
    }

    table
}
