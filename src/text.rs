use crate::style::Style;
use std::{borrow::Cow, cmp::max};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Fragment<'a> {
    Raw(Cow<'a, str>),
    Styled(Cow<'a, str>, Style),
}

impl<'a> Fragment<'a> {
    pub fn styled<S>(s: S, style: Style) -> Fragment<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Fragment::Styled(s.into(), style)
    }

    pub fn width(&self) -> usize {
        match self {
            Fragment::Raw(s) => s.width(),
            Fragment::Styled(s, _) => s.width(),
        }
    }

    pub fn style(&self) -> Style {
        match self {
            Fragment::Raw(_) => Style::default(),
            Fragment::Styled(_, s) => *s,
        }
    }
}

impl<'a> From<String> for Fragment<'a> {
    fn from(s: String) -> Fragment<'a> {
        Fragment::Raw(Cow::from(s))
    }
}

impl<'a> From<&'a str> for Fragment<'a> {
    fn from(s: &'a str) -> Fragment<'a> {
        Fragment::Raw(Cow::from(s))
    }
}

impl<'a> AsRef<str> for Fragment<'a> {
    fn as_ref(&self) -> &str {
        match self {
            Fragment::Raw(s) => s.as_ref(),
            Fragment::Styled(s, _) => s.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line<'a> {
    fragments: Vec<Fragment<'a>>,
}

impl<'a> Default for Line<'a> {
    fn default() -> Line<'a> {
        Line {
            fragments: Vec::new(),
        }
    }
}

impl<'a> From<String> for Line<'a> {
    fn from(s: String) -> Line<'a> {
        Line::with_fragments(vec![Fragment::from(s)])
    }
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Line<'a> {
        Line::with_fragments(vec![Fragment::from(s)])
    }
}

impl<'a> From<Vec<Fragment<'a>>> for Line<'a> {
    fn from(fragments: Vec<Fragment<'a>>) -> Line<'a> {
        Line { fragments }
    }
}

impl<'a> From<Fragment<'a>> for Line<'a> {
    fn from(fragment: Fragment<'a>) -> Line<'a> {
        Line {
            fragments: vec![fragment],
        }
    }
}

impl<'a> Line<'a> {
    pub fn with_fragments<T>(fragments: T) -> Line<'a>
    where
        T: Into<Vec<Fragment<'a>>>,
    {
        Line {
            fragments: fragments.into(),
        }
    }

    pub fn width(&self) -> usize {
        self.fragments.iter().fold(0, |acc, s| acc + s.width())
    }

    pub fn fragments(&self) -> &[Fragment<'a>] {
        &self.fragments
    }
}

impl<'a> From<Line<'a>> for String {
    fn from(line: Line<'a>) -> String {
        line.fragments.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(s.as_ref());
            acc
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    lines: Vec<Line<'a>>,
}

impl<'a> Default for Text<'a> {
    fn default() -> Text<'a> {
        Text { lines: Vec::new() }
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(lines: Vec<Line<'a>>) -> Text<'a> {
        Text { lines }
    }
}

impl<'a> Text<'a> {
    pub fn with_lines<T>(lines: T) -> Text<'a>
    where
        T: Into<Vec<Line<'a>>>,
    {
        Text {
            lines: lines.into(),
        }
    }

    pub fn width(&self) -> usize {
        self.lines.iter().fold(0, |acc, l| max(acc, l.width()))
    }
}
