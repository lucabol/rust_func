use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

// The Computer Language Benchmarks Game
// https://salsa.debian.org/benchmarksgame-team/benchmarksgame/
//
// contributed by Matt Watson
// contributed by TeXitoi
// contributed by Volodymyr M. Lisivka
// contributed by Michael Cicotti

extern crate generic_array;
extern crate num_traits;
extern crate numeric_array;
extern crate rayon;

use generic_array::typenum::consts::U8;
use numeric_array::NumericArray as Arr;
use rayon::prelude::*;

// [f64;8]
type Vecf64 = Arr<f64, U8>;
type Constf64 = numeric_array::NumericConstant<f64>;

const MAX_ITER: usize = 50;
const VLEN: usize = 8;

#[inline(always)]
pub fn mbrot8(out: &mut u8, cr: Vecf64, ci: Constf64) {
    let mut zr = Arr::splat(0f64);
    let mut zi = Arr::splat(0f64);
    let mut tr = Arr::splat(0f64);
    let mut ti = Arr::splat(0f64);
    let mut absz = Arr::splat(0f64);

    for _ in 0..MAX_ITER / 5 {
        for _ in 0..5 {
            zi = (zr + zr) * zi + ci;
            zr = tr - ti + cr;
            tr = zr * zr;
            ti = zi * zi;
        }

        absz = tr + ti;
        if absz.iter().all(|&t| t > 4.) {
            return;
        }
    }

    *out = absz.iter().enumerate().fold(0, |accu, (i, &t)| {
        accu | if t <= 4. { 0x80 >> i } else { 0 }
    });
}

fn mandel() -> Result<Response<String>, warp::http::Error> {
    let size = 16000;
    // Round size to multiple of 8
    let size = size / VLEN * VLEN;

    let inv = 2. / size as f64;

    let mut xloc = vec![Arr::splat(0f64); size / VLEN];
    for i in 0..size {
        xloc[i / VLEN][i % VLEN] = i as f64 * inv - 1.5;
    }

    let mut rows = vec![0; size * size / VLEN];
    rows.par_chunks_mut(size / VLEN)
        .enumerate()
        .for_each(|(y, out)| {
            let ci = numeric_array::NumericConstant(y as f64 * inv - 1.);
            out.iter_mut()
                .enumerate()
                .for_each(|(i, inner_out)| mbrot8(inner_out, xloc[i], ci));
        });

    Response::builder().body(rows.len().to_string())
    //Response::builder().body("Success!!".to_string())
}

#[tokio::main]
async fn main() {
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("mandelbrot"))
        .map(|| mandel());
    //.map(|| Response::builder().body("Success!!".to_string()));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1)
        .run((Ipv4Addr::UNSPECIFIED, port))
        .await
}
