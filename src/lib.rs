//! # modm - a library for masterminds

/// add two numbers together
///
/// ```
/// let arg = 5;
/// let ans = modm::add_one(arg);
///
/// assert_eq!(6, ans);
///
/// ```
pub fn add_one(arg: i32) -> i32 {
    arg + 1
}
