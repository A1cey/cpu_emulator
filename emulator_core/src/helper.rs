use core::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct FmtArray<'a, T>(pub &'a [T]);

impl<T> Deref for FmtArray<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
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
