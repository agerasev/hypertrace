#[cfg(feature = "host_tests")]
#[cfg(test)]
mod tests;
mod source;

pub use source::SOURCE;
pub use ocl_include as includer;
