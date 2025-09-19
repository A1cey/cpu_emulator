use core::{
    fmt::{Display, Formatter},
    ops::Deref,
};

/// A helper struct for formatting arrays.
///
/// # Example:
/// ```ignore
/// let array = [1, 2, 3];
/// let formatted = FmtArray(&array);
/// assert_eq!(formatted.to_string(), "[1, 2, 3]");
/// assert_eq!(format!("{}", formatted), "[1, 2, 3]");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FmtArray<'a, T>(pub &'a [T]);

impl<T> Deref for FmtArray<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Display> Display for FmtArray<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "[")?;

        let mut iter = self.iter();
        if let Some(val) = iter.next() {
            write!(f, "{val}")?;

            for val in iter {
                write!(f, ", {val}")?;
            }
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use alloc::string::ToString;

    #[test]
    fn test_fmt_array_with_to_string() {
        let array = [1, 2, 3];
        let formatted = FmtArray(&array);
        assert_eq!(formatted.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn test_fmt_array_with_format_macro() {
        let array = [1, 2, 3];
        let formatted = FmtArray(&array);
        assert_eq!(format!("{}", formatted), "[1, 2, 3]");
    }
}
