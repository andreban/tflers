// SPDX-License-Identifier: Apache-2.0

#[cfg(all(feature = "chrono", not(feature = "time")))]
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(all(feature = "time", not(feature = "chrono")))]
pub type DateTime = time::OffsetDateTime;

#[cfg(all(not(feature = "chrono"), not(feature = "time")))]
pub type DateTime = String;

#[cfg(all(feature = "chrono", feature = "time"))]
pub type DateTime = chrono::DateTime<chrono::Utc>;
