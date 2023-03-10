// Copyright (c) 2023 Joining7943 <joining@posteo.de>
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use TimeUnit::*;

/// The default identifier of [`TimeUnit::NanoSecond`]
pub const DEFAULT_ID_NANO_SECOND: &str = "ns";
/// The default identifier of [`TimeUnit::MicroSecond`]
pub const DEFAULT_ID_MICRO_SECOND: &str = "Ms";
/// The default identifier of [`TimeUnit::MicroSecond`]
pub const DEFAULT_ID_MILLI_SECOND: &str = "ms";
/// The default identifier of [`TimeUnit::Second`]
pub const DEFAULT_ID_SECOND: &str = "s";
/// The default identifier of [`TimeUnit::Minute`]
pub const DEFAULT_ID_MINUTE: &str = "m";
/// The default identifier of [`TimeUnit::Hour`]
pub const DEFAULT_ID_HOUR: &str = "h";
/// The default identifier of [`TimeUnit::Day`]
pub const DEFAULT_ID_DAY: &str = "d";
/// The default identifier of [`TimeUnit::Week`]
pub const DEFAULT_ID_WEEK: &str = "w";
/// The default identifier of [`TimeUnit::Month`]
pub const DEFAULT_ID_MONTH: &str = "M";
/// The default identifier of [`TimeUnit::Year`]
pub const DEFAULT_ID_YEAR: &str = "y";

pub(crate) const DEFAULT_TIME_UNIT: TimeUnit = Second;

/// The time units the parser can understand and needed to configure the [`DurationParser`].
///
/// # Examples
///
/// ```rust
/// use fundu::{DurationParser, TimeUnit};
/// use std::time::Duration;
///
/// assert_eq!(
///     DurationParser::with_time_units(&[TimeUnit::NanoSecond]).parse("42ns").unwrap(),
///     Duration::new(0, 42)
/// );
/// ```
///
/// [`DurationParser`]: crate::DurationParser
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum TimeUnit {
    /// Represents the lowest possible time unit. The default id is given by [`DEFAULT_ID_NANO_SECOND`] = `ns`
    NanoSecond,
    /// The default id is given by [`DEFAULT_ID_MICRO_SECOND`] = `Ms`
    MicroSecond,
    /// The default id is given by [`DEFAULT_ID_MILLI_SECOND`] = `ms`
    MilliSecond,
    /// The default if no time unit is given. The default id is given by [`DEFAULT_ID_SECOND`] = `s`
    Second,
    /// The default id is given by [`DEFAULT_ID_MINUTE`] = `m`
    Minute,
    /// The default id is given by [`DEFAULT_ID_HOUR`] = `h`
    Hour,
    /// The default id is given by [`DEFAULT_ID_DAY`] = `d`
    Day,
    /// The default id is given by [`DEFAULT_ID_WEEK`] = `w`
    Week,
    /// The default id is given by [`DEFAULT_ID_MONTH`] = `M`
    Month,
    /// Represents the hightest possible time unit. The default id is given by [`DEFAULT_ID_YEAR`] = `y`
    Year,
}

impl Default for TimeUnit {
    fn default() -> Self {
        DEFAULT_TIME_UNIT
    }
}

impl TimeUnit {
    /// Return the default identifier
    pub const fn default_identifier(&self) -> &'static str {
        match self {
            NanoSecond => DEFAULT_ID_NANO_SECOND,
            MicroSecond => DEFAULT_ID_MICRO_SECOND,
            MilliSecond => DEFAULT_ID_MILLI_SECOND,
            Second => DEFAULT_ID_SECOND,
            Minute => DEFAULT_ID_MINUTE,
            Hour => DEFAULT_ID_HOUR,
            Day => DEFAULT_ID_DAY,
            Week => DEFAULT_ID_WEEK,
            Month => DEFAULT_ID_MONTH,
            Year => DEFAULT_ID_YEAR,
        }
    }

    /// Return the multiplier to convert the number with [`TimeUnit`] to seconds.
    ///
    /// The multipliers change their application depending on whether the [`TimeUnit`] is less than,
    /// equal or greater than `seconds`:
    ///
    /// ```text
    /// t <= s => x(t) * 10^-m
    /// t > s  => x(t) * m
    /// where t = time unit, s = second, x = number in t time units, m = multiplier
    /// ```
    pub(crate) const fn multiplier(&self) -> (u64, i32) {
        match self {
            NanoSecond => (1, -9),
            MicroSecond => (1, -6),
            MilliSecond => (1, -3),
            Second => (1, 0),
            Minute => (60, 0),
            Hour => (3600, 0),
            Day => (86400, 0),
            Week => (604800, 0),
            Month => (2629800, 0), // Year / 12
            Year => (31557600, 0), // 365.25 days
        }
    }
}

pub trait TimeUnitsLike {
    fn is_empty(&self) -> bool;
    fn get(&self, identifier: &str) -> Option<TimeUnit>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::nano_second(NanoSecond, "ns")]
    #[case::micro_second(MicroSecond, "Ms")]
    #[case::milli_second(MilliSecond, "ms")]
    #[case::second(Second, "s")]
    #[case::minute(Minute, "m")]
    #[case::hour(Hour, "h")]
    #[case::day(Day, "d")]
    #[case::week(Week, "w")]
    #[case::month(Month, "M")]
    #[case::year(Year, "y")]
    fn test_time_unit_default_identifier(#[case] time_unit: TimeUnit, #[case] expected: &str) {
        assert_eq!(time_unit.default_identifier(), expected);
    }

    #[rstest]
    #[case::nano_second(NanoSecond, (1, -9))]
    #[case::micro_second(MicroSecond, (1, -6))]
    #[case::milli_second(MilliSecond, (1, -3))]
    #[case::second(Second, (1, 0))]
    #[case::minute(Minute, (60, 0))]
    #[case::hour(Hour, (60 * 60, 0))]
    #[case::day(Day, (60 * 60 * 24, 0))]
    #[case::week(Week, (60 * 60 * 24 * 7, 0))]
    #[case::month(Month, ((60 * 60 * 24 * 365 + 60 * 60 * 24 / 4) / 12, 0))] // (365 days + day/4) / 12
    #[case::year(Year, (60 * 60 * 24 * 365 + 60 * 60 * 24 / 4, 0))] // 365 days + day/4
    fn test_time_unit_multiplier(#[case] time_unit: TimeUnit, #[case] expected: (u64, i32)) {
        assert_eq!(time_unit.multiplier(), expected);
    }
}
