//! Normalizes text into Unix (\n) or DOS (\r\n) newline formats, using fast SIMD search and zero-copy when possible.
//!
//! Optimized for speed and minimal allocations. Returns a `Cow<str>` to avoid copies
//! when no change is needed.
//!
//! Supports both Unix (`\n`) and DOS (`\r\n`) style normalization.

/// Converts any mix of CRLF (`\r\n`) and CR (`\r`) newlines to LF (`\n`).
///
/// - Leaves input untouched if no carriage return is found.
/// - Unicode is preserved as-is. Grapheme clusters, RTL markers, and emoji remain intact.
///
/// Example:
/// ```
/// use newline_normalizer::ToUnixNewlines;
///
/// let text = "👩‍💻\r\nnaïve\rüber";
/// let normalized = text.to_unix_newlines();
/// assert_eq!(normalized, "👩‍💻\nnaïve\nüber");
/// ```
pub trait ToUnixNewlines {
    /// Normalize all line breaks in the input to LF (`\n`).
    ///
    /// Returns a borrowed reference if no transformation is needed.
    fn to_unix_newlines(&self) -> std::borrow::Cow<str>;
}

/// Converts any mix of LF (`\n`) and CR (`\r`) newlines to CRLF (`\r\n`).
///
/// - Leaves input untouched if all newlines are already CRLF.
/// - Unicode is preserved as-is. Grapheme clusters, RTL markers, and emoji remain intact.
///
/// Example:
/// ```
/// use newline_normalizer::ToDosNewlines;
///
/// let text = "مرحبا\nüber\r👨‍🔧";
/// let normalized = text.to_dos_newlines();
/// assert_eq!(normalized, "مرحبا\r\nüber\r\n👨‍🔧");
/// ```
pub trait ToDosNewlines {
    /// Normalize all line breaks in the input to CRLF (`\r\n`).
    ///
    /// Returns a borrowed reference if no transformation is needed.
    fn to_dos_newlines(&self) -> std::borrow::Cow<str>;
}

impl ToUnixNewlines for str {
    fn to_unix_newlines(&self) -> std::borrow::Cow<str> {
        let slice = self.as_bytes();
        let len = slice.len();
        let end_index = len.saturating_sub(1);
        let mut iter = memchr::memchr_iter(b'\r', slice);

        let Some(mut cr) = iter.next() else {
            return std::borrow::Cow::Borrowed(self);
        };

        let mut out = Vec::with_capacity(len);
        let mut pos = 0;

        loop {
            out.extend_from_slice(&slice[pos..cr]);
            out.push(b'\n');

            pos = cr + 1;
            if cr < end_index && slice[pos] == b'\n' {
                pos += 1;
            }

            match iter.next() {
                Some(next_cr) => cr = next_cr,
                None => break,
            }
        }

        if pos < len {
            out.extend_from_slice(&slice[pos..]);
        }

        std::borrow::Cow::Owned(unsafe { String::from_utf8_unchecked(out) })
    }
}

impl ToDosNewlines for str {
    fn to_dos_newlines(&self) -> std::borrow::Cow<str> {
        let slice = self.as_bytes();
        let len = slice.len();
        let end_index = len.saturating_sub(1);
        let mut iter = memchr::memchr2_iter(b'\n', b'\r', slice);

        // Skip all properly formatted CRLF pairs
        let mut crlf = usize::MAX;
        while let Some(match_pos) = iter.next() {
            if (slice[match_pos] == b'\r' && match_pos < end_index && slice[match_pos + 1] == b'\n') ||
                (slice[match_pos] == b'\n' && match_pos > 0 && slice[match_pos - 1] == b'\r')  {
                continue;
            }
            crlf = match_pos;
            break;
        }

        if crlf == usize::MAX {
            return std::borrow::Cow::Borrowed(self);
        }

        let mut out = Vec::with_capacity(len);
        let mut pos = 0;
        let mut current;

        loop {
            if crlf >= pos {
                out.extend_from_slice(&slice[pos..crlf]);
                out.extend_from_slice(&[b'\r', b'\n']);
                current = slice[crlf];
                pos = crlf + 1;
                
                if current == b'\r' && crlf < end_index && slice[pos] == b'\n' {
                    pos += 1;
                }
            }

            match iter.next() {
                Some(next_crlf) => crlf = next_crlf,
                None => break,
            }
        }
        
        if pos < len {
            out.extend_from_slice(&slice[pos..]);
        }

        std::borrow::Cow::Owned(unsafe { String::from_utf8_unchecked(out) })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod unix_newlines_tests {
        use std::borrow::Cow;

        use super::ToUnixNewlines;
        
        #[test]
        fn preserves_lf_only_input() {
            assert_eq!(
                "line1\nline2\nline3".to_unix_newlines(), "line1\nline2\nline3"
            );
        }

        #[test]
        fn converts_crlf_to_lf() {
            assert_eq!(
                "line1\r\nline2\r\nline3".to_unix_newlines(), "line1\nline2\nline3"
            );
        }

        #[test]
        fn converts_cr_to_lf() {
            assert_eq!(
                "line1\rline2\rline3".to_unix_newlines(), "line1\nline2\nline3"
            );
        }

        #[test]
        fn handles_mixed_newlines() {
            assert_eq!(
                "line1\rline2\r\nline3\nline4\r".to_unix_newlines(), "line1\nline2\nline3\nline4\n"
            );
        }

        #[test]
        fn empty_input_returns_borrowed() {
            assert_eq!("".to_unix_newlines(), "");
        }

        #[test]
        fn single_line_crlf() {
            assert_eq!(
                "line\r\n".to_unix_newlines(), "line\n"
            );
        }

        #[test]
        fn single_line_cr() {
            assert_eq!(
                "line\r".to_unix_newlines(), "line\n"
            );
        }

        #[test]
        fn handles_double_cr() {
            assert_eq!(
                "line\r\rline2".to_unix_newlines(), "line\n\nline2"
            );
        }

        #[test]
        fn preserves_unicode_accents() {
            let input = "élève\r\nüber\rcoöperate\nnaïve";
            let expected = "élève\nüber\ncoöperate\nnaïve";
            assert_eq!(input.to_unix_newlines(), expected);
        }

        #[test]
        fn preserves_rtl_text_newlines() {
            let input = "مرحبا\r\nبالعالم\rمرحبا\nبكم";
            let expected = "مرحبا\nبالعالم\nمرحبا\nبكم";
            assert_eq!(input.to_unix_newlines(), expected);
        }

        #[test]
        fn preserves_combining_characters() {
            let input = "a\u{0301}\r\nb\u{0323}\r";
            let expected = "a\u{0301}\nb\u{0323}\n";
            assert_eq!(input.to_unix_newlines(), expected);
        }

        #[test]
        fn preserves_emoji_sequences() {
            let input = "👩‍💻\r\n👨‍🔧\r👩\n";
            let expected = "👩‍💻\n👨‍🔧\n👩\n";
            assert_eq!(input.to_unix_newlines(), expected);
        }

        #[test]
        fn trailing_carriage_return_only() {
            let input = "line1\rline2\r";
            let expected = "line1\nline2\n";
            assert_eq!(input.to_unix_newlines(), expected);
        }

        #[test]
        fn embedded_fullwidth_characters() {
            let input = "a\u{3000}b\r\nc\u{200B}d\r";
            let expected = "a\u{3000}b\nc\u{200B}d\n";
            assert_eq!(input.to_unix_newlines(), expected);
        }

        #[test]
        fn avoid_allocating_for_normal_string() {
            let input = "This is already a normal string,\nno need to run normalizer.\n";
            let result = input.to_unix_newlines();
            assert!(matches!(result, Cow::Borrowed(_)));
            assert_eq!(result, result);
        }
    }

    #[cfg(test)]
    mod dos_newlines_tests {
        use std::borrow::Cow;
        use super::ToDosNewlines;

        #[test]
        fn preserves_crlf_only_input() {
            assert_eq!(
                "line1\r\nline2\r\nline3".to_dos_newlines(),
                "line1\r\nline2\r\nline3"
            );
        }

        #[test]
        fn handles_lf_after_crlf() {
            assert_eq!(
                "Это пример параграфа с пробелами и юникодом.\r\n\n    Он".to_dos_newlines(),
                "Это пример параграфа с пробелами и юникодом.\r\n\r\n    Он"
            );
        }

        #[test]
        fn handles_lf_at_start() {
            assert_eq!(
                "\nЭто пример параграфа с пробелами и юникодом.".to_dos_newlines(),
                "\r\nЭто пример параграфа с пробелами и юникодом."
            );
        }


        #[test]
        fn converts_lf_to_crlf() {
            assert_eq!(
                "line1\nline2\nline3".to_dos_newlines(),
                "line1\r\nline2\r\nline3"
            );
        }

        #[test]
        fn converts_cr_to_crlf() {
            assert_eq!(
                "line1\rline2\rline3".to_dos_newlines(),
                "line1\r\nline2\r\nline3"
            );
        }

        #[test]
        fn handles_mixed_newlines() {
            assert_eq!(
                "line1\r\nline2\rline3\nline4\r".to_dos_newlines(),
                "line1\r\nline2\r\nline3\r\nline4\r\n"
            );
        }

        #[test]
        fn empty_input_returns_borrowed() {
            assert_eq!("".to_dos_newlines(), "");
        }

        #[test]
        fn single_line_lf() {
            assert_eq!(
                "line\n".to_dos_newlines(),
                "line\r\n"
            );
        }

        #[test]
        fn single_line_cr() {
            assert_eq!(
                "line\r".to_dos_newlines(),
                "line\r\n"
            );
        }

        #[test]
        fn preserves_unicode_accents() {
            let input = "élève\nüber\rcoöperate\r\nnaïve";
            let expected = "élève\r\nüber\r\ncoöperate\r\nnaïve";
            assert_eq!(input.to_dos_newlines(), expected);
        }

        #[test]
        fn preserves_rtl_text_newlines() {
            let input = "مرحبا\nبالعالم\rمرحبا\r\nبكم";
            let expected = "مرحبا\r\nبالعالم\r\nمرحبا\r\nبكم";
            assert_eq!(input.to_dos_newlines(), expected);
        }

        #[test]
        fn preserves_combining_characters() {
            let input = "a\u{0301}\nb\u{0323}\r";
            let expected = "a\u{0301}\r\nb\u{0323}\r\n";
            assert_eq!(input.to_dos_newlines(), expected);
        }

        #[test]
        fn preserves_emoji_sequences() {
            let input = "👩‍💻\n👨‍🔧\r👩‍🔬\r\n";
            let expected = "👩‍💻\r\n👨‍🔧\r\n👩‍🔬\r\n";
            assert_eq!(input.to_dos_newlines(), expected);
        }

        #[test]
        fn trailing_lone_cr_only() {
            let input = "line1\rline2\r";
            let expected = "line1\r\nline2\r\n";
            assert_eq!(input.to_dos_newlines(), expected);
        }

        #[test]
        fn embedded_fullwidth_characters() {
            let input = "a\u{3000}b\nc\u{200B}d\r";
            let expected = "a\u{3000}b\r\nc\u{200B}d\r\n";
            assert_eq!(input.to_dos_newlines(), expected);
        }

        #[test]
        fn avoid_allocating_for_normal_string() {
            let input = "\r\nThis is already a normal string,\r\nno need to run normalizer.\r\n";
            let result = input.to_dos_newlines();
            assert!(matches!(result, Cow::Borrowed(_)));
            assert_eq!(result, result);
        }
    }
}