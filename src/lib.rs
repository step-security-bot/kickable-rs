// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

//! > **kickable** is a package created to answer the age old question... "_Can I Kick It?_"
//!
//! Quick Links:
//! - Can I Kick It [music video](https://www.youtube.com/watch?v=O3pyCGnZzYA)
//! ## Example
//!
//! Run
//! ```console
//! $ cargo add kickable
//! ```
//! Then use kickable in your code`:
#![allow(rustdoc::bare_urls)]
#![cfg_attr(not(feature = "derive"), doc = " ```ignore")]
#![doc = include_str!("../examples/kickable.rs")]
pub use anyhow::Result;
pub mod args;
pub mod i18n;

/// Returns true if the input supplied is kickable.
///
/// # Arguments
///
/// * `input` - A string to validate for kick-ability.
///
pub fn validate(input: &str) -> bool {
    if input.trim().to_lowercase() == "it" {
        return true;
    }
    false
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn validate_should_pass_it() {
        let result = validate("it");
        assert_eq!(result, true);
    }
    #[test]
    fn validate_should_pass_it_upper() {
        let result = validate("IT");
        assert_eq!(result, true);
    }
    #[test]
    fn validate_should_pass_it_padded() {
        let result = validate(" it ");
        assert_eq!(result, true);
    }
    #[test]
    fn validate_should_fail_empty() {
        let result = validate("");
        assert_eq!(result, false);
    }
    #[test]
    fn validate_should_fail_other() {
        let result = validate("other");
        assert_eq!(result, false);
    }
}
