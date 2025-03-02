use wasm_bindgen::prelude::*;

mod tools;
mod vis;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    tools::gen(seed as u64, "A").to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(input: String, output: String, turn: usize) -> Ret {
    let input = tools::parse_input(&input);

    // // turn を考慮
    // let out = tools::parse_output(&input, &output, turn);
    let out = tools::parse_output(&input, &output);

    let (score, err, svg) = match out {
        Ok(out) => {
            let (score, err) = tools::compute_score(&input, &out);

            if err.len() > 0 {
                (0, err, "".to_string())
            } else {
                let (_score, err, svg) = vis::vis(&input, &out, turn);

                if err.len() > 0 {
                    (0, err, "".to_string())
                } else {
                    (score, err, svg)
                }
            }
        }
        Err(err) => (0, err, "".to_string()),
    };

    if err.len() > 0 {
        return Ret {
            score: 0,
            err,
            svg: "".to_string(),
        };
    }

    Ret { score, err, svg }
}

#[wasm_bindgen]
pub fn get_max_turn(input: String, output: String) -> usize {
    let input = tools::parse_input(&input);
    let out = tools::parse_output(&input, &output);

    match out {
        Ok(out) => out.out.len(),
        Err(_err) => 0,
    }
}
