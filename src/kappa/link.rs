use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Link {
    Unknown,
    Free,
    Numbered(usize),
    Bound,
    BoundTo { agent: String, site: String },
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Link::Unknown => f.write_char('#'),
            Link::Free => f.write_char('.'),
            Link::Numbered(n) => n.fmt(f),
            Link::Bound => f.write_char('_'),
            Link::BoundTo { site, agent } => write!(f, "{}.{}", site, agent),
        }
    }
}
