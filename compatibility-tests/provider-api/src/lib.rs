#![cfg(test)]
#![feature(error_generic_member_access, provide_any)]

use snafu::{prelude::*, Backtrace, IntoError};

#[test]
fn runtime_fields_can_be_provided() {
    #[derive(Debug, Snafu)]
    struct WithArbitraryRuntimeError {
        #[snafu(provide)]
        name: String,
    }

    let e = WithArbitraryRuntimeSnafu { name: "bob" }.build();
    let e = &e as &dyn snafu::Error;
    let inner = e.request_ref::<String>();

    assert!(matches!(inner.map(String::as_str), Some("bob")));
}

#[test]
fn constant_fields_can_be_provided() {
    #[derive(Debug, Snafu)]
    #[snafu(provide(i32 => 100))]
    struct WithArbitraryConstantError;

    let e = WithArbitraryConstantSnafu.build();
    let e = &e as &dyn snafu::Error;
    let inner = e.request_value::<i32>();

    assert!(matches!(inner, Some(100)));
}

#[test]
fn constant_fields_can_use_expressions() {
    #[derive(Debug, Snafu)]
    #[snafu(provide(u8 => ALPHA + beta::gamma() + Delta::default().epsilon()))]
    struct WithExpressionConstantError;

    const ALPHA: u8 = 1;
    mod beta {
        pub fn gamma() -> u8 {
            1
        }
    }
    #[derive(Default)]
    struct Delta;
    impl Delta {
        fn epsilon(&self) -> u8 {
            1
        }
    }

    let e = WithExpressionConstantSnafu.build();
    let e = &e as &dyn snafu::Error;
    let inner = e.request_value::<u8>();

    assert!(matches!(inner, Some(3)));
}

#[test]
fn sources_are_automatically_provided() {
    #[derive(Debug, Snafu)]
    struct InnerError;

    #[derive(Debug, Snafu)]
    struct WithSourceError {
        source: InnerError,
    }

    let e = WithSourceSnafu.into_error(InnerError);
    let e = &e as &dyn snafu::Error;
    let inner = e.request_ref::<InnerError>();

    assert!(matches!(inner, Some(InnerError)));
}

#[test]
fn backtraces_are_automatically_provided() {
    #[derive(Debug, Snafu)]
    struct WithBacktraceError {
        backtrace: Backtrace,
    }

    let e = WithBacktraceSnafu.build();
    let e = &e as &dyn snafu::Error;
    let bt = e.request_ref::<Backtrace>();

    assert!(bt.is_some());
}

// opaque error passes along?
// enum and struct?
// test for duplicated provider
// disable a default provider
// chain to source error
// specify if field is before / after source chained
// override if provided field is reference or value
// override provided field type (e.g. &String -> &str)
// implicit fields
// Option<Backtrace> (maybe a transformer that can handle `&string` -> `&str` too?)
