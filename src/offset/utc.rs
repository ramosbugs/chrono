// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! The UTC (Coordinated Universal Time) time zone.

use core::fmt;

#[cfg(any(feature = "rkyv-16", feature = "rkyv-32", feature = "rkyv-64"))]
use rkyv::{Archive, Deserialize, Serialize};

use super::{FixedOffset, MappedLocalTime, Offset, TimeZone};
use crate::naive::NaiveDateTime;
#[cfg(all(feature = "now", doc))]
use crate::OutOfRange;

/// The UTC time zone. This is the most efficient time zone when you don't need the local time.
/// It is also used as an offset (which is also a dummy type).
///
/// Using the [`TimeZone`](./trait.TimeZone.html) methods
/// on the UTC struct is the preferred way to construct `DateTime<Utc>`
/// instances.
///
/// # Example
///
/// ```
/// use chrono::{DateTime, TimeZone, Utc};
///
/// let dt = DateTime::from_timestamp(61, 0).unwrap();
///
/// assert_eq!(Utc.at_timestamp(61, 0).unwrap(), dt);
/// assert_eq!(Utc.at_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap(), dt);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    any(feature = "rkyv-16", feature = "rkyv-32", feature = "rkyv-64"),
    derive(Archive, Deserialize, Serialize),
    archive(compare(PartialEq)),
    archive_attr(derive(Clone, Copy, PartialEq, Eq, Debug, Hash))
)]
#[cfg_attr(feature = "rkyv-validation", archive(check_bytes))]
#[cfg_attr(all(feature = "arbitrary", feature = "std"), derive(arbitrary::Arbitrary))]
pub struct Utc;

#[cfg(feature = "now")]
impl Utc {
    /// Returns a `DateTime<Utc>` which corresponds to the current date and time in UTC.
    ///
    /// See also the similar [`Local::now()`] which returns `DateTime<Local>`, i.e. the local date
    /// and time including offset from UTC.
    ///
    /// [`Local::now()`]: crate::Local::now
    ///
    /// # Example
    ///
    /// ```
    /// # #![allow(unused_variables)]
    /// # use chrono::{FixedOffset, Utc};
    /// // Current time in UTC
    /// let now_utc = Utc::now();
    ///
    /// // Current date in UTC
    /// let today_utc = now_utc.date_naive();
    ///
    /// // Current time in some timezone (let's use +05:00)
    /// let offset = FixedOffset::east(5 * 60 * 60).unwrap();
    /// let now_with_offset = Utc::now().with_timezone(&offset);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the system clock is set to a time in the extremely distant past or future, such
    /// that it is out of the range representable by `DateTime<Utc>`. It is assumed that this
    /// crate will no longer be in use by that time.
    // Covers the platforms with `SystemTime::time()` supported by the Rust Standard Library as of
    // Rust 1.78. See:
    //   https://github.com/rust-lang/rust/blob/22a5267c83a3e17f2b763279eb24bb632c45dc6b/library/std/src/sys/pal/uefi/mod.rs
    // Note that some platforms listed in the PAL table do not support `SystemTime::time()` (e.g.,
    // `zkvm` and `wasm`).
    #[cfg(any(
        unix,
        windows,
        target_os = "solid_asp3",
        target_os = "hermit",
        target_os = "wasi",
        target_os = "xous",
        all(target_vendor = "fortanix", target_env = "sgx"),
        target_os = "teeos",
    ))]
    #[must_use]
    pub fn now() -> crate::DateTime<Utc> {
        crate::DateTime::try_from_system_time(std::time::SystemTime::now()).expect(
            "system clock is set to a time extremely far into the past or future; cannot convert",
        )
    }

    /// Returns a `DateTime` which corresponds to the current date and time.
    #[cfg(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))]
    #[must_use]
    pub fn now() -> crate::DateTime<Utc> {
        let now = js_sys::Date::new_0();
        crate::DateTime::<Utc>::from(now)
    }
}

impl TimeZone for Utc {
    type Offset = Utc;

    fn from_offset(_state: &Utc) -> Utc {
        Utc
    }

    fn offset_from_local_datetime(&self, _local: NaiveDateTime) -> MappedLocalTime<Utc> {
        MappedLocalTime::Single(Utc)
    }

    fn offset_from_utc_datetime(&self, _utc: NaiveDateTime) -> Utc {
        Utc
    }
}

impl Offset for Utc {
    fn fix(&self) -> FixedOffset {
        FixedOffset::east(0).unwrap()
    }
}

impl fmt::Debug for Utc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Z")
    }
}

impl fmt::Display for Utc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UTC")
    }
}
