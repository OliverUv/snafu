//! Exposing complete backtraces to the location of the error.
//!
//! Start by looking at the error type [`Error`].

use crate::{Snafu, Backtrace, ErrorCompat, GenerateBacktrace};

/// Backtraces aren't yet supported by the stable Rust compiler. SNAFU
/// provides a stable-compatible way of getting backtraces as well as
/// support for the backtrace support in the nightly compiler. **By
/// default, backtraces are disabled**. It is
/// expected that the final executable adds SNAFU as a dependency and
/// chooses the appropriate [feature
/// flag](crate::guide::feature_flags) to enable backtraces.
///
/// When using SNAFU to define error types, it's recommended to start
/// with a [`Backtrace`] field on every leaf error variant (those
/// without a `source`). Backtraces are only captured on
/// failure. Since backtraces are disabled by default, adding them in
/// a library does not force any users to pay the price of backtraces
/// if they are not used; they can be zero cost.
///
/// Certain errors are used for flow control. Those don't need a
/// backtrace as they don't represent actual failures. However,
/// sometimes an error is *mostly* used for flow control but might
/// also indicate an error. In those cases, you can use
/// `Option<Backtrace>` to avoid capturing a backtrace unless an
/// environment variable is set by the end user to provide additional
/// debugging.
///
/// For variants that do have a source, you need to evaluate if the
/// source error provides a backtrace of some kind. If it is another
/// SNAFU error, for example, you can *delegate* retrieval of the
/// backtrace to the source error. If the source error doesn't provide
/// its own backtrace, you should capture your own backtrace. This
/// backtrace would not be as useful as one captured by the source
/// error, but it's as useful as you can get.
///
/// When you wish to display the backtrace of an error, you can use
/// the [`ErrorCompat::backtrace`] method. It's recommended to always
/// use this in the fully-qualified form so it will be easy to find
/// and replace when Rust stabilizes backtraces.
///
/// ```
/// # use snafu::guide::examples::backtrace::*;
/// use snafu::ErrorCompat;
///
/// fn inner_process() -> Result<(), Error> {
///     // Complicated logic
///     # UsualCaseSnafu.fail()
/// }
///
/// fn main() {
///     if let Err(e) = inner_process() {
///         eprintln!("An error occurred: {}", e);
///         if let Some(bt) = ErrorCompat::backtrace(&e) {
///             eprintln!("{:?}", bt);
///         }
///     }
/// }
/// ```
#[derive(Debug, Snafu)]
// This line is only needed to generate documentation; it is not
// needed in most cases:
#[snafu(crate_root(crate), visibility = "pub")]
pub enum Error {
    /// The most common leaf error should always include a backtrace field.    .
    UsualCase {
        backtrace: Backtrace,
    },

    /// When an error is expected to be created frequently but the
    /// backtrace is rarely needed, you can wrap it in an
    /// `Option`. See
    /// [GenerateBacktrace](GenerateBacktrace#impl-GenerateBacktrace-for-Option<Backtrace>)
    /// for instructions on how to access the backtrace in this
    /// case.q.
    UsedInTightLoop {
        backtrace: Option<Backtrace>,
    },

    /// This error wraps another error that already has a
    /// backtrace. Instead of capturing our own, we forward the
    /// request for the backtrace to the inner error. This gives a
    /// more accurate backtrace.
    SnafuErrorAsSource {
        #[snafu(backtrace)]
        source: ConfigFileError,
    },

    /// This error wraps another error that does not expose a
    /// backtrace. We capture our own backtrace to provide something
    /// useful.
    SourceErrorDoesNotHaveBacktrace {
        source: std::io::Error,
        backtrace: Backtrace,
    },
}

/// This is a placeholder example and can be ignored.
#[derive(Debug, Snafu)]
#[snafu(crate_root(crate))]
pub enum ConfigFileError {
    Dummy { backtrace: Backtrace },
}
