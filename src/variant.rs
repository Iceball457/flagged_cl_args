use std::{
    fmt::Display,
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
    str::FromStr,
};

/// Variant flag determines what types an argument is allowed to become!
/// If the argument can become a string, parsing it will never fail, but it will only become a string if it can't become any of the other types it is allowed to.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VariantFlag(u8);

impl VariantFlag {
    const BOOL_BIT: u8 = 0;
    const INT_BIT: u8 = 1;
    const FLOAT_BIT: u8 = 2;
    const SOCKET_BIT: u8 = 3;
    const PATH_BIT: u8 = 4;
    const STRING_BIT: u8 = 5;

    // #[must_use]
    // pub fn new(
    //     bool_allowed: bool,
    //     int_allowed: bool,
    //     float_allowed: bool,
    //     string_allowed: bool,
    // ) -> VariantFlag {
    //     VariantFlag(
    //         if bool_allowed {
    //             1 << VariantFlag::BOOL_BIT
    //         } else {
    //             0
    //         } + if int_allowed {
    //             1 << VariantFlag::INT_BIT
    //         } else {
    //             0
    //         } + if float_allowed {
    //             1 << VariantFlag::FLOAT_BIT
    //         } else {
    //             0
    //         } + if string_allowed {
    //             1 << VariantFlag::STRING_BIT
    //         } else {
    //             0
    //         },
    //     )
    // }

    /// The argument doesn't have a value. It is either present or it is not.
    /// If present, it will have a value of Variant::Bool(true).
    #[must_use]
    pub fn new_unit() -> VariantFlag {
        VariantFlag(0)
    }

    /// An argument parsed with the resulting VariantFlag will only become a [`bool`].
    /// Booleans are parsed exclusively from 'true' and 'false'.
    #[must_use]
    pub fn bool() -> VariantFlag {
        VariantFlag(1 << VariantFlag::BOOL_BIT)
    }

    /// Adds [`bool`] to the list of types an argument can support.
    /// Supports method chaining.
    #[must_use]
    pub fn or_bool(self) -> VariantFlag {
        VariantFlag(1 << VariantFlag::BOOL_BIT | self.0)
    }

    /// An argument parsed with the resulting VariantFlag will only become an [`i32`].
    #[must_use]
    pub fn int() -> VariantFlag {
        VariantFlag(1 << VariantFlag::INT_BIT)
    }

    /// Adds [`i32`] to the list of types an argument can support.
    /// Supports method chaining.
    #[must_use]
    pub fn or_int(self) -> VariantFlag {
        VariantFlag(1 << VariantFlag::INT_BIT | self.0)
    }

    /// An argument parsed with the resulting VariantFlag will only become an [`f32`].
    #[must_use]
    pub fn float() -> VariantFlag {
        VariantFlag(1 << VariantFlag::FLOAT_BIT)
    }

    /// An argument parsed with the resulting VariantFlag will only become an [`std::net::SocketAddr`].
    #[must_use]
    pub fn socket() -> VariantFlag {
        VariantFlag(1 << VariantFlag::SOCKET_BIT)
    }

    /// Adds [`std::net::SocketAddr`] to the list of types an argument can support.
    /// Supports method chaining.
    #[must_use]
    pub fn or_socket(self) -> VariantFlag {
        VariantFlag(1 << VariantFlag::SOCKET_BIT | self.0)
    }

    /// An argument parsed with the resulting VariantFlag will only become an [`std::path::PathBuf`].
    /// This conversion will never fail, but that doesn't mean the path points to anything meaningful.
    #[must_use]
    pub fn path() -> VariantFlag {
        VariantFlag(1 << VariantFlag::PATH_BIT)
    }

    /// Adds [`std::path::Path`] to the list of types an argument can support.
    /// Supports method chaining.
    #[must_use]
    pub fn or_path(self) -> VariantFlag {
        VariantFlag(1 << VariantFlag::PATH_BIT | self.0)
    }

    /// An argument parsed with the resulting VariantFlag will be passed directly as a [`String`].
    /// This conversion will never fail.
    #[must_use]
    pub fn string() -> VariantFlag {
        VariantFlag(1 << VariantFlag::STRING_BIT)
    }

    /// Adds [`String`] to the list of types an argument can support.
    /// Supports method chaining.
    #[must_use]
    pub fn or_string(self) -> VariantFlag {
        VariantFlag(1 << VariantFlag::STRING_BIT | self.0)
    }

    #[must_use]
    fn check_bit(&self, bit: u8) -> bool {
        self.0 >> bit & 1 != 0
    }

    #[must_use]
    pub(crate) fn bool_allowed(&self) -> bool {
        self.check_bit(VariantFlag::BOOL_BIT)
    }

    #[must_use]
    pub(crate) fn int_allowed(&self) -> bool {
        self.check_bit(VariantFlag::INT_BIT)
    }

    #[must_use]
    pub(crate) fn float_allowed(&self) -> bool {
        self.check_bit(VariantFlag::FLOAT_BIT)
    }

    #[must_use]
    pub(crate) fn socket_allowed(&self) -> bool {
        self.check_bit(VariantFlag::SOCKET_BIT)
    }

    #[must_use]
    pub(crate) fn path_allowed(&self) -> bool {
        self.check_bit(VariantFlag::PATH_BIT)
    }

    #[must_use]
    pub(crate) fn string_allowed(&self) -> bool {
        self.check_bit(VariantFlag::STRING_BIT)
    }

    #[must_use]
    pub(crate) fn is_unit(&self) -> bool {
        self.0 == 0
    }

    /// Parse a string into one of the types this VariantFlag supports.
    /// The precedence is bool, i32, f32, SocketAddr, PathBuf, and lastly String
    #[must_use]
    pub fn parse(&self, raw: &str) -> Option<Variant> {
        if self.bool_allowed()
            && let Ok(b) = bool::from_str(raw)
        {
            Some(Variant::Bool(b))
        } else if self.int_allowed()
            && let Ok(i) = i32::from_str(raw)
        {
            Some(Variant::Int(i))
        } else if self.float_allowed()
            && let Ok(f) = f32::from_str(raw)
        {
            Some(Variant::Float(f))
        } else if self.socket_allowed()
            && let Ok(mut sockets) = raw.to_socket_addrs()
            && let Some(s) = sockets.next()
        {
            Some(Variant::Socket(s))
        } else if self.path_allowed() {
            Some(Variant::Path(PathBuf::from(raw)))
        } else if self.string_allowed() {
            Some(Variant::String(raw.to_string()))
        } else {
            None
        }
    }
}

/// A value of a particular type.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Variant {
    /// Booleans are represented as [`bool`]
    Bool(bool),
    /// Integers are represented as [`i32`]
    Int(i32),
    /// Floats are represented as [`f32`]
    Float(f32),
    /// Sockets are represented as [`std::net::SocketAddr`]
    Socket(SocketAddr),
    /// Paths are represented as [`std::path::PathBuf`]
    Path(PathBuf),
    /// Strings are represented as [`String`]
    String(String),
}

impl Variant {
    /// Orders values according to their precedence. Values of the same type are sorted using Ord, or their specialized sorting function as needed.
    #[must_use]
    pub fn total_cmp(&self, other: &Variant) -> std::cmp::Ordering {
        let lhs = match self {
            Variant::Bool(_) => (0, self),
            Variant::Int(_) => (1, self),
            Variant::Float(_) => (2, self),
            Variant::Socket(_) => (3, self),
            Variant::Path(_) => (4, self),
            Variant::String(_) => (5, self),
        };
        let rhs = match other {
            Variant::Bool(_) => (0, other),
            Variant::Int(_) => (1, other),
            Variant::Float(_) => (2, other),
            Variant::Socket(_) => (3, self),
            Variant::Path(_) => (4, self),
            Variant::String(_) => (5, self),
        };
        if lhs.0 == rhs.0 {
            match (self, other) {
                (Variant::Bool(lhs), Variant::Bool(rhs)) => lhs.cmp(rhs),
                (Variant::Int(lhs), Variant::Int(rhs)) => lhs.cmp(rhs),
                (Variant::Float(lhs), Variant::Float(rhs)) => lhs.total_cmp(rhs),
                (Variant::Socket(lhs), Variant::Socket(rhs)) => lhs.cmp(rhs),
                (Variant::Path(lhs), Variant::Path(rhs)) => lhs.as_os_str().cmp(rhs.as_os_str()),
                (Variant::String(lhs), Variant::String(rhs)) => lhs.cmp(rhs),
                _ => unreachable!(),
            }
        } else {
            lhs.0.cmp(&rhs.0)
        }
    }

    /// Maps from Variant to Option\<bool\>
    pub fn as_bool(&self) -> Option<bool> {
        if let Variant::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Maps from Variant to Option\<i32\>
    pub fn as_int(&self) -> Option<i32> {
        if let Variant::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    /// Maps from Variant to Option\<f32\>
    pub fn as_float(&self) -> Option<f32> {
        if let Variant::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    /// Maps from Variant to Option\<SocketAddr\>
    pub fn as_socket(&self) -> Option<SocketAddr> {
        if let Variant::Socket(s) = self {
            Some(*s)
        } else {
            None
        }
    }

    /// Maps from Variant to Option\<&PathBuf\>
    pub fn as_path(&self) -> Option<&PathBuf> {
        if let Variant::Path(p) = self {
            Some(p)
        } else {
            None
        }
    }

    /// Maps from Variant to Option\<Pathbuf\>
    pub fn into_path(self) -> Option<PathBuf> {
        if let Variant::Path(p) = self {
            Some(p)
        } else {
            None
        }
    }

    /// Maps from Variant to Option\<&str\>
    pub fn as_string(&self) -> Option<&str> {
        if let Variant::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Maps from Variant to Option\<String\>
    pub fn into_string(self) -> Option<String> {
        if let Variant::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Bool(inner) => inner.fmt(f),
            Variant::Int(inner) => inner.fmt(f),
            Variant::Float(inner) => inner.fmt(f),
            Variant::Socket(inner) => inner.fmt(f),
            Variant::Path(inner) => inner.to_string_lossy().fmt(f),
            Variant::String(inner) => inner.fmt(f),
        }
    }
}

impl Display for VariantFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_unit() {
            return write!(f, "flag");
        }
        fn inner(
            first: &mut Option<&str>,
            val: &str,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            let res = write!(f, "{}{}", first.unwrap_or(""), val);
            *first = Some("|");
            res
        }
        let mut separator = None;
        if self.bool_allowed() {
            inner(&mut separator, "bool", f)?;
        }
        if self.int_allowed() {
            inner(&mut separator, "int", f)?;
        }
        if self.float_allowed() {
            inner(&mut separator, "float", f)?;
        }
        if self.socket_allowed() {
            inner(&mut separator, "socket", f)?;
        }
        if self.path_allowed() {
            inner(&mut separator, "path", f)?;
        }
        if self.string_allowed() {
            inner(&mut separator, "string", f)?;
        }
        Ok(())
    }
}
