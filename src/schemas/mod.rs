mod custom;

#[allow(
    dead_code,
    non_camel_case_types,
    unused_mut,
    unused_variables,
    clippy::never_loop,
    clippy::single_match,
    clippy::redundant_field_names
)]
#[rustfmt::skip]
mod generated;

pub use custom::*;
pub use generated::*;
