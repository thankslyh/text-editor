use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy)]
pub enum GraphemeWidth {
    Half,
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

#[derive(Debug, Clone)]
struct TextFragment {
    content: String,
    render_width: GraphemeWidth,
    replacement: Option<char>,

}

#[derive(Debug)]
pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = line_str.graphemes(true)
            .map(|s| {
                let width = s.width();
                let render_width = match width {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };
                let replacement = match width {
                    0 => Some('.'),
                    _ => None,
                };
                TextFragment {
                    render_width,
                    replacement,
                    content: s.to_string(),
                }
            })
            .collect::<Vec<TextFragment>>();
        Self {
            fragments
        }
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

    pub fn width_until(&self, index: usize) -> usize {
        self.fragments.iter().take(index).map(|c| match c.render_width {
            GraphemeWidth::Full => 2,
            GraphemeWidth::Half => 1,
        }).sum()
    }
}