#![feature(test)]

extern crate test;

use std::fs;
use test::Bencher;

#[bench]
fn test_bench_real(b: &mut Bencher) {
    let str = fs::read_to_string("../samples/jsonRPC.lsif").unwrap();
    b.iter(|| {
        lsif_parser::parser::parse(&str);
    });
}

#[bench]
fn test_bench(b: &mut Bencher) {
    let str = "{ \"id\": 5, \"type\": \"edge\", \"label\": \"contains\", \"outV\": 1, \"inVs\": [4] }";
    // let str = "{ id: 5, type: \"edge\", label: \"contains\", outV: 1, inVs: [4] }";

    b.iter(|| {
        lsif_parser::parser::parse(&str);
    });
}
