mod safety_factor;
mod block;

use safety_factor::{SafetyFactor, SafetyFactorTrait};
use block::Block;

fn main() {

    let mut block_list = Vec::<Block>::new();

    let mut alpha_s = [62.8, 44.6, 33.0, 24.0, 17.0, 8.9, 2.2, 16.0, 21.1, 34.0];
    for i in 0..10 {
        alpha_s[i] = alpha_s[i] / 360.0 * 2.0 * std::f64::consts::PI;
    }

    let hs = [35.2, 33.3, 31.8, 42.3, 42.1, 41.6, 42.3, 42.0, 41.3, 37.0];


    for i in 0..10 {
        block_list.push(Block {
            phi: 0.,
            alpha: alpha_s[i],
            b: (40. - 15.) / 10.0,
            c: get_c_from_height(hs[i]),
            w: hs[i] * get_weight_from_hight(hs[i]),
        });
    }

    let pre_safety_factor = SafetyFactor {
        safety_factor: 200.0,
        blocks: block_list,
    };

    let estated_safety_factor = pre_safety_factor.estate().estate();

    println!("safety factor: {}", estated_safety_factor);

    let sample_height = [34.0, 31.0, 28.0, 23.0, 22.0, 20.0, 18.0];
    for h in sample_height {
        let gamma_d = get_weight_from_hight(h) / (1.0 + get_w_from_hight(h));

        let e = 2.65 * 9.81 / gamma_d - 1.0;

        println!("height: {}\t e: {}", h, e);
    }
}

fn get_w_from_hight(h: f64) -> f64 {
    // 土の含水比 %
    match h {
        h if h >= 34.0 => 0.40,
        h if h >= 31.0 => 1.10,
        h if h >= 28.0 => 1.05,
        h if h >= 26.0 => 0.50,
        h if h >= 23.0 => 0.60,
        h if h >= 22.0 => 0.40,
        h if h >= 18.0 => 0.40,
        _ => 0.40,
    }
}

fn get_weight_from_hight(h: f64) -> f64 {
    // 土の単位体積重量 kN/m3
    match h {
        h if h >= 34.0 => 18.0,
        h if h >= 31.0 => 14.2,
        h if h >= 28.0 => 15.0,
        h if h >= 23.0 => 16.0,
        h if h >= 22.0 => 17.0,
        h if h >= 18.0 => 18.0,
        _ => 19.0,
    }
}

fn get_c_from_height(h: f64) -> f64 {
    // 土の粘着力 kPa
    match h {
        h if h >= 34.0 => 50.0 / 2.,
        h if h >= 31.0 => 20.0 / 2.,
        h if h >= 23.0 => 20.0 / 2.,
        h if h >= 22.0 => 30.0 / 2.,
        h if h >= 20.0 => 40.0 / 2.,
        _ => 55.0 / 2.,
    }
}