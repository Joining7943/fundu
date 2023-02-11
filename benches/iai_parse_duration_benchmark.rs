// Copyright (c) 2023 Joining7943 <joining@posteo.de>
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use fundu::DurationParser;
use iai::black_box;

const LARGE_INPUT: &str = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111.11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111e-1022";
const SMALL_INPUT: &str = "1";
const SMALL_INPUT_SECONDS: &str = "1s";
const SMALL_INPUT_MICRO_SECONDS: &str = "1Ms";

fn small_default_time_units() {
    let _ = DurationParser::new().parse(black_box(SMALL_INPUT));
}

fn small_without_time_units() {
    let _ = DurationParser::without_time_units().parse(black_box(SMALL_INPUT));
}

fn large_default_time_units() {
    let _ = DurationParser::new().parse(black_box(LARGE_INPUT));
}

fn large_without_time_units() {
    let _ = DurationParser::without_time_units().parse(black_box(LARGE_INPUT));
}

fn small_default_time_units_when_second() {
    let _ = DurationParser::new().parse(black_box(SMALL_INPUT_SECONDS));
}

fn small_default_time_units_when_micro_second() {
    let _ = DurationParser::new().parse(black_box(SMALL_INPUT_MICRO_SECONDS));
}

fn small_reference() {
    let _ = Duration::from_secs_f64(black_box(SMALL_INPUT).parse().unwrap());
}

fn large_reference() {
    let _ = Duration::from_secs_f64(black_box(LARGE_INPUT).parse().unwrap());
}

iai::main!(
    small_default_time_units,
    small_without_time_units,
    small_reference,
    large_default_time_units,
    large_without_time_units,
    large_reference,
    small_default_time_units_when_second,
    small_default_time_units_when_micro_second
);
