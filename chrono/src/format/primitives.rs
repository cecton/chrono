//extern crate std as core;

/// Fixed-format item types.
///
/// They have their own rules of formatting and parsing.
/// Otherwise noted, they print in the specified cases but parse case-insensitively.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Fixed {
    /// Abbreviated month names.
    ///
    /// Prints a three-letter-long name in the title case, reads the same name in any case.
    ShortMonthName,
    /// Full month names.
    ///
    /// Prints a full name in the title case, reads either a short or full name in any case.
    LongMonthName,
    /// Abbreviated day of the week names.
    ///
    /// Prints a three-letter-long name in the title case, reads the same name in any case.
    ShortWeekdayName,
    /// Full day of the week names.
    ///
    /// Prints a full name in the title case, reads either a short or full name in any case.
    LongWeekdayName,
    /// AM/PM.
    ///
    /// Prints in lower case, reads in any case.
    LowerAmPm,
    /// AM/PM.
    ///
    /// Prints in upper case, reads in any case.
    UpperAmPm,
    /// An optional dot plus one or more digits for left-aligned nanoseconds.
    /// May print nothing, 3, 6 or 9 digits according to the available accuracy.
    /// See also [`Numeric::Nanosecond`](./enum.Numeric.html#variant.Nanosecond).
    Nanosecond,
    /// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3.
    Nanosecond3,
    /// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6.
    Nanosecond6,
    /// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9.
    Nanosecond9,
    /// Timezone name.
    ///
    /// It does not support parsing, its use in the parser is an immediate failure.
    TimezoneName,
    /// Offset from the local time to UTC (`+09:00` or `-04:00` or `+00:00`).
    ///
    /// In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
    /// The offset is limited from `-24:00` to `+24:00`,
    /// which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.
    TimezoneOffsetColon,
    /// Offset from the local time to UTC (`+09:00` or `-04:00` or `Z`).
    ///
    /// In the parser, the colon can be omitted and/or surrounded with any amount of whitespace,
    /// and `Z` can be either in upper case or in lower case.
    /// The offset is limited from `-24:00` to `+24:00`,
    /// which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.
    TimezoneOffsetColonZ,
    /// Same as [`TimezoneOffsetColon`](#variant.TimezoneOffsetColon) but prints no colon.
    /// Parsing allows an optional colon.
    TimezoneOffset,
    /// Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ) but prints no colon.
    /// Parsing allows an optional colon.
    TimezoneOffsetZ,
    /// RFC 2822 date and time syntax. Commonly used for email and MIME date and time.
    RFC2822,
    /// RFC 3339 & ISO 8601 date and time syntax.
    RFC3339,

    /// Internal uses only.
    ///
    /// This item exists so that one can add additional internal-only formatting
    /// without breaking major compatibility (as enum variants cannot be selectively private).
    Internal(InternalFixed),
}

/// An uninhabited type used for `InternalNumeric` and `InternalFixed` below.
#[derive(Clone, PartialEq, Eq)]
enum Void {}

/// Padding characters for numeric items.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pad {
    /// No padding.
    None,
    /// Zero (`0`) padding.
    Zero,
    /// Space padding.
    Space,
}

/// Numeric item types.
/// They have associated formatting width (FW) and parsing width (PW).
///
/// The **formatting width** is the minimal width to be formatted.
/// If the number is too short, and the padding is not [`Pad::None`](./enum.Pad.html#variant.None),
/// then it is left-padded.
/// If the number is too long or (in some cases) negative, it is printed as is.
///
/// The **parsing width** is the maximal width to be scanned.
/// The parser only tries to consume from one to given number of digits (greedily).
/// It also trims the preceding whitespace if any.
/// It cannot parse the negative number, so some date and time cannot be formatted then
/// parsed with the same formatting items.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Numeric {
    /// Full Gregorian year (FW=4, PW=∞).
    /// May accept years before 1 BCE or after 9999 CE, given an initial sign.
    Year,
    /// Gregorian year divided by 100 (century number; FW=PW=2). Implies the non-negative year.
    YearDiv100,
    /// Gregorian year modulo 100 (FW=PW=2). Cannot be negative.
    YearMod100,
    /// Year in the ISO week date (FW=4, PW=∞).
    /// May accept years before 1 BCE or after 9999 CE, given an initial sign.
    IsoYear,
    /// Year in the ISO week date, divided by 100 (FW=PW=2). Implies the non-negative year.
    IsoYearDiv100,
    /// Year in the ISO week date, modulo 100 (FW=PW=2). Cannot be negative.
    IsoYearMod100,
    /// Month (FW=PW=2).
    Month,
    /// Day of the month (FW=PW=2).
    Day,
    /// Week number, where the week 1 starts at the first Sunday of January (FW=PW=2).
    WeekFromSun,
    /// Week number, where the week 1 starts at the first Monday of January (FW=PW=2).
    WeekFromMon,
    /// Week number in the ISO week date (FW=PW=2).
    IsoWeek,
    /// Day of the week, where Sunday = 0 and Saturday = 6 (FW=PW=1).
    NumDaysFromSun,
    /// Day of the week, where Monday = 1 and Sunday = 7 (FW=PW=1).
    WeekdayFromMon,
    /// Day of the year (FW=PW=3).
    Ordinal,
    /// Hour number in the 24-hour clocks (FW=PW=2).
    Hour,
    /// Hour number in the 12-hour clocks (FW=PW=2).
    Hour12,
    /// The number of minutes since the last whole hour (FW=PW=2).
    Minute,
    /// The number of seconds since the last whole minute (FW=PW=2).
    Second,
    /// The number of nanoseconds since the last whole second (FW=PW=9).
    /// Note that this is *not* left-aligned;
    /// see also [`Fixed::Nanosecond`](./enum.Fixed.html#variant.Nanosecond).
    Nanosecond,
    /// The number of non-leap seconds since the midnight UTC on January 1, 1970 (FW=1, PW=∞).
    /// For formatting, it assumes UTC upon the absence of time zone offset.
    Timestamp,

    /// Internal uses only.
    ///
    /// This item exists so that one can add additional internal-only formatting
    /// without breaking major compatibility (as enum variants cannot be selectively private).
    Internal(InternalNumeric),
}

/// An opaque type representing numeric item types for internal uses only.
pub struct InternalNumeric {
    _dummy: Void,
}

impl Clone for InternalNumeric {
    fn clone(&self) -> Self {
        match self._dummy {}
    }
}

impl PartialEq for InternalNumeric {
    fn eq(&self, _other: &InternalNumeric) -> bool {
        match self._dummy {}
    }
}

impl Eq for InternalNumeric {}

impl core::fmt::Debug for InternalNumeric {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "<InternalNumeric>")
    }
}

/// An opaque type representing fixed-format item types for internal uses only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalFixed {
    val: InternalInternal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InternalInternal {
    /// Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ), but
    /// allows missing minutes (per [ISO 8601][iso8601]).
    ///
    /// # Panics
    ///
    /// If you try to use this for printing.
    ///
    /// [iso8601]: https://en.wikipedia.org/wiki/ISO_8601#Time_offsets_from_UTC
    TimezoneOffsetPermissive,
    /// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3 and there is no leading dot.
    Nanosecond3NoDot,
    /// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6 and there is no leading dot.
    Nanosecond6NoDot,
    /// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9 and there is no leading dot.
    Nanosecond9NoDot,
}

/// A single formatting item. This is used for both formatting and parsing.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item<'a> {
    /// A literally printed and parsed text.
    Literal(&'a str),
    /// Same as `Literal` but with the string owned by the item.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    OwnedLiteral(Box<str>),
    /// Whitespace. Prints literally but reads zero or more whitespace.
    Space(&'a str),
    /// Same as `Space` but with the string owned by the item.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    OwnedSpace(Box<str>),
    /// Numeric item. Can be optionally padded to the maximal length (if any) when formatting;
    /// the parser simply ignores any padded whitespace and zeroes.
    Numeric(Numeric, Pad),
    /// Fixed-format item.
    Fixed(Fixed),
    /// Issues a formatting error. Used to signal an invalid format string.
    Error,
}
