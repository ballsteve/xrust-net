/*!
A helper library for [Xrust](https://crates.io/crates/xrust).

This create provides a function to fetch data, given a URL. It is used for xsl:include and xsl:import statements.
 */

pub mod resolver;
pub use resolver::resolve;

