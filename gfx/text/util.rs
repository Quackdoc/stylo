/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use text::glyph::CharIndex;

#[derive(PartialEq, Eq, Copy)]
pub enum CompressionMode {
    CompressNone,
    CompressWhitespace,
    CompressWhitespaceNewline,
    DiscardNewline
}

// ported from Gecko's nsTextFrameUtils::TransformText.
//
// High level TODOs:
//
// * Issue #113: consider incoming text state (arabic, etc)
//               and propagate outgoing text state (dual of above)
//
// * Issue #114: record skipped and kept chars for mapping original to new text
//
// * Untracked: various edge cases for bidi, CJK, etc.
pub fn transform_text(text: &str,
                      mode: CompressionMode,
                      incoming_whitespace: bool,
                      output_text: &mut String,
                      new_line_pos: &mut Vec<CharIndex>)
                      -> bool {
    let out_whitespace = match mode {
        CompressionMode::CompressNone | CompressionMode::DiscardNewline => {
            let mut new_line_index = CharIndex(0);
            for ch in text.chars() {
                if is_discardable_char(ch, mode) {
                    // TODO: record skipped char
                } else {
                    // TODO: record kept char
                    if ch == '\t' {
                        // TODO: set "has tab" flag
                    } else if ch == '\n' {
                        // Save new-line's position for line-break
                        // This value is relative(not absolute)
                        new_line_pos.push(new_line_index);
                        new_line_index = CharIndex(0);
                    }

                    if ch != '\n' {
                        new_line_index = new_line_index + CharIndex(1);
                    }
                    output_text.push(ch);
                }
            }
            text.len() > 0 && is_in_whitespace(text.char_at_reverse(0), mode)
        },

        CompressionMode::CompressWhitespace | CompressionMode::CompressWhitespaceNewline => {
            let mut in_whitespace: bool = incoming_whitespace;
            for ch in text.chars() {
                // TODO: discard newlines between CJK chars
                let mut next_in_whitespace: bool = is_in_whitespace(ch, mode);

                if !next_in_whitespace {
                    if is_always_discardable_char(ch) {
                        // revert whitespace setting, since this char was discarded
                        next_in_whitespace = in_whitespace;
                        // TODO: record skipped char
                    } else {
                        // TODO: record kept char
                        output_text.push(ch);
                    }
                } else { /* next_in_whitespace; possibly add a space char */
                    if in_whitespace {
                        // TODO: record skipped char
                    } else {
                        // TODO: record kept char
                        output_text.push(' ');
                    }
                }
                // save whitespace context for next char
                in_whitespace = next_in_whitespace;
            } /* /for str::each_char */
            in_whitespace
        }
    };

    return out_whitespace;

    fn is_in_whitespace(ch: char, mode: CompressionMode) -> bool {
        match (ch, mode) {
            (' ', _)  => true,
            ('\t', _) => true,
            ('\n', CompressionMode::CompressWhitespaceNewline) => true,
            (_, _)    => false
        }
    }

    fn is_discardable_char(ch: char, mode: CompressionMode) -> bool {
        if is_always_discardable_char(ch) {
            return true;
        }
        match mode {
            CompressionMode::DiscardNewline | CompressionMode::CompressWhitespaceNewline => ch == '\n',
            _ => false
        }
    }

    fn is_always_discardable_char(_ch: char) -> bool {
        // TODO: check for bidi control chars, soft hyphens.
        false
    }
}

pub fn float_to_fixed(before: usize, f: f64) -> i32 {
    ((1i32 << before) as f64 * f) as i32
}

pub fn fixed_to_float(before: usize, f: i32) -> f64 {
    f as f64 * 1.0f64 / ((1i32 << before) as f64)
}

pub fn fixed_to_rounded_int(before: isize, f: i32) -> isize {
    let half = 1i32 << (before-1) as usize;
    if f > 0i32 {
        ((half + f) >> before) as isize
    } else {
       -((half - f) >> before) as isize
    }
}
