mod safety_factor;
mod block;

use safety_factor::{SafetyFactor, SafetyFactorTrait};
use block::Block;

fn main() {

    let mut block_list = Vec::<Block>::new();

    let mut alpha_s = [-46., -23., -13., -6., -1., -1., 0., 8.];
    for i in 0..8 {
        alpha_s[i] = alpha_s[i] / 360.0 * 2.0 * std::f64::consts::PI;
    }

    let hs = [35.8, 34.5, 33.7, 33.3, 33.2, 33.0, 33.0, 33.5];

    let width = (35. - 20.) / 8.0;

    for i in 0..8 {
        block_list.push(Block {
            phi: 0.,
            alpha: alpha_s[i],
            b: width,
            c: get_c_from_height(hs[i]),
            w: calculate_w(hs[i]),
        });
    }

    let pre_safety_factor = SafetyFactor {
        safety_factor: 1.8,
        blocks: block_list,
    };

    let estated_safety_factor = pre_safety_factor.estate();

    println!("safety factor: {}", estated_safety_factor);

    return;

    let sample_height = [34.0, 31.0, 28.0, 23.0, 22.0, 20.0, 18.0];
    for h in sample_height {
        let gamma_d = get_weight_from_hight(h) / (1.0 + get_w_from_hight(h));

        let e = 2.65 * 9.81 / gamma_d - 1.0;

        println!("height: {}\t e: {}", h, e);
    }
}

fn calculate_w(h: f64) -> f64 {
    let mut w = 0.0;

    let mut i = 0.0;
    while i <= h {
        w += get_weight_from_hight(37.-i);
        i += 0.1;
    }
    
    return w / 10.;
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
        h if h >= 34.0 => 40.0,
        h if h >= 31.0 => 20.0,
        h if h >= 23.0 => 20.0,
        h if h >= 22.0 => 30.0,
        h if h >= 20.0 => 40.0,
        _ => 55.0,
    }
}