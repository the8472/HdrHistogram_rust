#![feature(test)]

extern crate hdrsample;
extern crate rand;
extern crate test;

use std::time;

use hdrsample::*;
use hdrsample::serialization;
use hdrsample::serialization::interval_log;
use test::Bencher;

use self::rand_varint::*;

#[path = "../src/serialization/rand_varint.rs"]
mod rand_varint;

#[bench]
fn write_interval_log_1k_hist_10k_value(b: &mut Bencher) {
    let mut log = Vec::new();
    let mut histograms = Vec::new();
    let mut rng = rand::weak_rng();

    for _ in 0..1000 {
        let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

        for v in RandomVarintEncodedLengthIter::new(&mut rng).take(10_000) {
            h.record(v).unwrap();
        }

        histograms.push(h);
    }

    let mut serializer = serialization::V2Serializer::new();

    b.iter(|| {
        log.clear();

        let mut writer = interval_log::IntervalLogWriterBuilder::new()
            .build_with(&mut log, &mut serializer)
            .unwrap();

        let dur = time::Duration::new(5, 678_000_000);
        for h in histograms.iter() {
            writer.write_histogram(h, 1.234, dur, None).unwrap();
        }
    })
}

#[bench]
fn parse_interval_log_1k_hist_10k_value(b: &mut Bencher) {
    let mut log = Vec::new();
    let mut histograms = Vec::new();
    let mut rng = rand::weak_rng();

    for _ in 0..1000 {
        let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

        for v in RandomVarintEncodedLengthIter::new(&mut rng).take(10_000) {
            h.record(v).unwrap();
        }

        histograms.push(h);
    }

    {
        let mut serializer = serialization::V2Serializer::new();
        let mut writer = interval_log::IntervalLogWriterBuilder::new()
            .build_with(&mut log, &mut serializer)
            .unwrap();

        let dur = time::Duration::new(5, 678_000_000);
        for h in histograms.iter() {
            writer.write_histogram(h, 1.234, dur, None).unwrap();
        }
    }

    b.iter(|| {
        let iter = interval_log::IntervalLogIterator::new(&log);

        assert_eq!(1000, iter.count());
    })
}
