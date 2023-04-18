mod source;
#[cfg(feature = "host_tests")]
#[cfg(test)]
mod tests;

pub use ocl_include as includer;
pub use source::SOURCE;
