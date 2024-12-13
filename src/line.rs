use std::{fmt::Display, ops::Range};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy, Default)]
pub enum GraphemeWidth {
    Half,
    #[default]
    Full,
}

impl GraphemeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            GraphemeWidth::Half => other.saturating_add(1),
            GraphemeWidth::Full => other.saturating_add(2),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct TextFragment {
    content: String,
    render_width: GraphemeWidth,
    replacement: Option<char>,
}

#[derive(Debug, Default)]
pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            fragments: Self::str_to_fragments(line_str),
        }
    }

    fn str_to_fragments(str: &str) -> Vec<TextFragment> {
        str.graphemes(true)
            .map(|s| {
                let (replacement, render_width) = Self::replacement_character(s).map_or_else(
                    || {
                        let unicode_width = s.width();
                        let rendered_width = match unicode_width {
                            0 | 1 => GraphemeWidth::Half,
                            _ => GraphemeWidth::Full,
                        };
                        (None, rendered_width)
                    },
                    |replacement| (Some(replacement), GraphemeWidth::Half),
                );
                TextFragment {
                    render_width,
                    replacement,
                    content: s.to_string(),
                }
            })
            .collect::<Vec<TextFragment>>()
    }
    pub fn get(&self, range: Range<usize>) -> String {
        if range.start > range.end {
            return String::new();
        }
        let mut result = String::new();
        let mut current_pos = 0;
        for str in &self.fragments {
            let str_end = str.render_width.saturating_add(current_pos);
            if current_pos >= range.end {
                break;
            }
            if str_end > range.start {
                if str_end > range.end || current_pos < range.start {
                    result.push('⋯');
                } else if let Some(replacement) = str.replacement {
                    result.push(replacement);
                } else {
                    result.push_str(&str.content);
                }
            }
            current_pos = str_end
        }
        result
    }

    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    fn replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }

    pub fn width_until(&self, index: usize) -> usize {
        self.fragments
            .iter()
            .take(index)
            .map(|c| match c.render_width {
                GraphemeWidth::Full => 2,
                GraphemeWidth::Half => 1,
            })
            .sum()
    }

    pub fn intert_char(&mut self, s: char, idx: usize) {
        let mut result = String::new();
        for (index, str) in self.fragments.iter().enumerate() {
            if idx == index {
                result.push(s);
            }
            result.push_str(&str.content);
        }
        if idx >= self.len() {
            result.push(s);
        }
        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn delete(&mut self, idx: usize) {
        let mut result = String::new();
        for (i, str) in self.fragments.iter().enumerate() {
            if i != idx {
                result.push_str(&str.content.to_string());
            }
        }
        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn append(&mut self, other: &Self) {
        let mut tmp_str = self.to_string();
        tmp_str.push_str(&other.to_string());
        self.fragments = Self::str_to_fragments(&tmp_str);
    }

    pub fn split(&mut self, at: usize) -> Self {
        if at >= self.len() {
            return Self::default();
        }
        let result = self.fragments.split_off(at);
        Self { fragments: result }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String = self.fragments.iter().map(|s| s.content.clone()).collect();
        write!(f, "{tmp}")
    }
}
