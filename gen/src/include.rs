use crate::gen::out::OutFile;
use crate::syntax::IncludeKind;
use std::fmt::{self, Display};

/// The complete contents of the "rust/cxx.h" header.
pub static HEADER: &str = include_str!("include/cxx.h");

pub(super) fn write(out: &mut OutFile, needed: bool, guard: &str) {
    let ifndef = format!("#ifndef {}", guard);
    let define = format!("#define {}", guard);
    let endif = format!("#endif // {}", guard);

    let mut offset = 0;
    loop {
        let begin = find_line(offset, &ifndef);
        let end = find_line(offset, &endif);
        if let (Some(begin), Some(end)) = (begin, end) {
            if !needed {
                return;
            }
            out.next_section();
            if offset == 0 {
                writeln!(out, "{}", ifndef);
                writeln!(out, "{}", define);
            }
            for line in HEADER[begin + ifndef.len()..end].trim().lines() {
                if line != define && !line.trim_start().starts_with("//") {
                    writeln!(out, "{}", line);
                }
            }
            offset = end + endif.len();
        } else if offset == 0 {
            panic!("not found in cxx.h header: {}", guard)
        } else {
            writeln!(out, "{}", endif);
            return;
        }
    }
}

fn find_line(mut offset: usize, line: &str) -> Option<usize> {
    loop {
        offset += HEADER[offset..].find(line)?;
        let rest = &HEADER[offset + line.len()..];
        if rest.starts_with('\n') || rest.starts_with('\r') {
            return Some(offset);
        }
        offset += line.len();
    }
}

/// A header to #include.
///
/// The cxxbridge tool does not parse or even require the given paths to exist;
/// they simply go into the generated C++ code as #include lines.
#[derive(Clone, PartialEq, Debug)]
pub struct Include {
    /// The header's path, not including the enclosing quotation marks or angle
    /// brackets.
    pub path: String,
    /// Whether to emit `#include "path"` or `#include <path>`.
    pub kind: IncludeKind,
}

#[derive(Default, PartialEq)]
pub struct Includes {
    custom: Vec<Include>,
    pub array: bool,
    pub cstddef: bool,
    pub cstdint: bool,
    pub cstring: bool,
    pub exception: bool,
    pub memory: bool,
    pub new: bool,
    pub string: bool,
    pub type_traits: bool,
    pub utility: bool,
    pub vector: bool,
    pub basetsd: bool,
}

impl Includes {
    pub fn new() -> Self {
        Includes::default()
    }

    pub fn insert(&mut self, include: Include) {
        self.custom.push(include);
    }
}

impl<'a> Extend<&'a Include> for Includes {
    fn extend<I: IntoIterator<Item = &'a Include>>(&mut self, iter: I) {
        self.custom.extend(iter.into_iter().cloned());
    }
}

impl Display for Includes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for include in &self.custom {
            match include.kind {
                IncludeKind::Quoted => {
                    writeln!(f, "#include \"{}\"", include.path.escape_default())?;
                }
                IncludeKind::Bracketed => {
                    writeln!(f, "#include <{}>", include.path)?;
                }
            }
        }
        if self.array {
            writeln!(f, "#include <array>")?;
        }
        if self.cstddef {
            writeln!(f, "#include <cstddef>")?;
        }
        if self.cstdint {
            writeln!(f, "#include <cstdint>")?;
        }
        if self.cstring {
            writeln!(f, "#include <cstring>")?;
        }
        if self.exception {
            writeln!(f, "#include <exception>")?;
        }
        if self.memory {
            writeln!(f, "#include <memory>")?;
        }
        if self.new {
            writeln!(f, "#include <new>")?;
        }
        if self.string {
            writeln!(f, "#include <string>")?;
        }
        if self.type_traits {
            writeln!(f, "#include <type_traits>")?;
        }
        if self.utility {
            writeln!(f, "#include <utility>")?;
        }
        if self.vector {
            writeln!(f, "#include <vector>")?;
        }
        if self.basetsd {
            writeln!(f, "#if defined(_WIN32)")?;
            writeln!(f, "#include <basetsd.h>")?;
            writeln!(f, "#endif")?;
        }
        Ok(())
    }
}
