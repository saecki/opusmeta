use std::borrow::Cow;
use std::fmt::Display;
use std::ops::Deref;

/// A lowercase String. Holds a [`Cow<str>`] internally.
#[derive(Debug, Clone)]
pub struct LowercaseString<'a>(pub(crate) Cow<'a, str>);

impl Deref for LowercaseString<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for LowercaseString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<'a> LowercaseString<'a> {
    /// Create a new `LowercaseString`. This will only allocate if the passed
    /// string isn't lowercase.
    pub fn from_str(str: &'a str) -> Self {
        if !str.chars().any(|c| c.is_ascii_uppercase()) {
            Self(Cow::Borrowed(str))
        } else {
            Self(Cow::Owned(str.to_ascii_lowercase()))
        }
    }

    /// Create a new `LowercaseString`. If the string isn't already lowercase,
    /// this will modify the existing buffer without allocating.
    pub fn from_string(mut str: String) -> Self {
        str.make_ascii_lowercase();
        Self(Cow::Owned(str))
    }

    /// Try to create a new `LowercaseString`. This returns `None` if the passed
    /// string isn't lowercase.
    pub const fn try_from_str(str: &'a str) -> Option<Self> {
        // for loops and iterator/trait methods aren't const stable yet.
        let mut i = 0;
        while i < str.as_bytes().len() {
            if str.as_bytes()[i].is_ascii_uppercase() {
                return None;
            }
            i += 1;
        }

        Some(Self(Cow::Borrowed(str)))
    }
}

impl<S: AsRef<str>> From<S> for LowercaseString<'static> {
    fn from(str: S) -> Self {
        Self::from_string(str.as_ref().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dont_allocate_already_lowercase_str() {
        let lower = LowercaseString::from_str("adsf-adsf");
        assert!(matches!(lower.0, Cow::Borrowed(_)))
    }
}
