use std::borrow::Cow;

const UNDERSCORE_CHAR: char = '_';

pub fn to_snakecase<'a, S>(s: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let input = s.into();
    let mut chars = input.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c.is_lowercase() || c.is_numeric() {
            continue;
        }
        if c == UNDERSCORE_CHAR {
            if let Some((_, c)) = chars.peek() {
                if c.is_lowercase() || c.is_numeric() {
                    chars.nth(0);
                    continue;
                }
            } else {
                // need to manipulate string '_' is the past character in the string
                // can return directly from here as we know to just strip the last char
                return Cow::Owned(input[..i].to_owned());
            }
        }
        // if we got here then we need to manipulate the string
        let mut result: String = String::with_capacity(input.len() + 5);
        result.push_str(&input[..i]);

        if !c.is_alphanumeric() {
            snakecase_mod(i > 0, &input, &mut result, &mut chars);
        } else if c.is_uppercase() {
            if i > 0 {
                result.push(UNDERSCORE_CHAR);
            }
            result.extend(c.to_lowercase());
            if let Some((_, c)) = chars.peek() {
                snakecase_mod(!c.is_alphanumeric(), &input, &mut result, &mut chars);
            }
        }
        return Cow::Owned(result);
    }
    input
}

fn snakecase_mod(
    add_underscore: bool,
    input: &str,
    result: &mut String,
    chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
) {
    while let Some((_, c)) = chars.next() {
        if !c.is_alphanumeric() {
            continue;
        }
        if add_underscore {
            result.push(UNDERSCORE_CHAR);
        }

        if c.is_uppercase() {
            result.extend(c.to_lowercase());
            while let Some((_, c)) = chars.peek() {
                if c.is_uppercase() {
                    result.extend(c.to_lowercase());
                    chars.next();
                    continue;
                }
                return snakecase_mod(!c.is_lowercase(), &input, result, chars);
            }
        }

        if c.is_lowercase() || c.is_numeric() {
            result.push(c);
            while let Some((_, c)) = chars.peek() {
                if c.is_lowercase() || c.is_numeric() {
                    result.push(*c);
                    chars.next();
                    continue;
                }
                return snakecase_mod(true, &input, result, chars);
            }
        }
    }
}

const UNDERSCORE_BYTE: u8 = b'_';

pub fn to_snakecase_ascii<'a, S>(s: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let input = s.into();
    let bytes = input.as_bytes();
    let mut idx = 0;

    // loop through all good characters:
    // - lowercase
    // - digit
    // - underscore (as long as the next character is lowercase or digit)
    while idx < bytes.len()
        && (is_lower_or_digit(bytes[idx])
            || (bytes[idx] == UNDERSCORE_BYTE
                && idx < bytes.len() - 1
                && is_lower_or_digit(bytes[idx + 1])))
    {
        idx += 1;
    }

    if idx >= bytes.len() {
        // '>=' performs much better than '==', I suspect it's due to bounds checking
        return input; // no changes needed, can just borrow the string
    }
    // if we get then we must need to manipulate the string
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len() + 5);
    // handles digit followed by an uppercase character to match a previous libraries functionality
    if idx > 0 && bytes[idx - 1].is_ascii_digit() {
        idx -= 1;
    }
    result.extend_from_slice(&bytes[..idx]);

    while idx < bytes.len() {
        if !bytes[idx].is_ascii_alphanumeric() {
            idx += 1;
            continue;
        }

        if !result.is_empty() {
            result.push(UNDERSCORE_BYTE);
        }

        while idx < bytes.len() && is_upper_or_digit_add(&mut result, bytes[idx]) {
            idx += 1;
        }

        while idx < bytes.len() && is_lower_or_digit(bytes[idx]) {
            result.push(bytes[idx]);
            idx += 1;
        }
    }
    // we know this is safe because prior to this we eliminated all non-ascii chars so we are guaranteed
    // to only have utf-8 at this point.
    Cow::Owned(unsafe { String::from_utf8_unchecked(result) })
}

#[inline]
fn is_upper_or_digit_add(result: &mut Vec<u8>, b: u8) -> bool {
    if b.is_ascii_uppercase() {
        result.push(b.to_ascii_lowercase());
        true
    } else if b.is_ascii_digit() {
        result.push(b);
        true
    } else {
        false
    }
}

#[inline]
fn is_lower_or_digit(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    macro_rules! snakecase_test {
        ($name:ident, $input:expr, $output:expr, $b:expr) => {
            #[test]
            fn $name() {
                let results = to_snakecase($input);
                assert_eq!(results, $output);
                assert_eq!(
                    match results {
                        Cow::Borrowed(_) => true,
                        _ => false,
                    },
                    $b
                );
            }
        };
    }

    snakecase_test!(empty, "", "", true);
    snakecase_test!(equal, "sample_text", "sample_text", true);
    snakecase_test!(space, "sample text", "sample_text", false);
    snakecase_test!(dash, "sample-text", "sample_text", false);
    snakecase_test!(multi_underscore, "sample___text", "sample_text", false);
    snakecase_test!(ending_underscore, "sample_text_", "sample_text", false);
    snakecase_test!(
        ending_multi_underscore,
        "sample_text__",
        "sample_text",
        false
    );
    snakecase_test!(uppercase_sep, "sampleText", "sample_text", false);
    snakecase_test!(
        multi_uppercase,
        "inviteYourCustomersAddInvites",
        "invite_your_customers_add_invites",
        false
    );
    snakecase_test!(
        space_with_uppercase,
        "sample 2 Text",
        "sample_2_text",
        false
    );
    snakecase_test!(special_chars, "FOO:BAR$BAZ", "foo_bar_baz", false);
    snakecase_test!(caps, "samPLE text", "sam_ple_text", false);
    snakecase_test!(
        multi_spaces,
        "   sample   2    Text   ",
        "sample_2_text",
        false
    );
    snakecase_test!(
        special_with_spaces,
        "   $#$sample   2    Text   ",
        "sample_2_text",
        false
    );
    snakecase_test!(caps_with_space_sep, "SAMPLE 2 TEXT", "sample_2_text", false);
    snakecase_test!(
        leading_underscore_special,
        "___$$Base64Encode",
        "base64_encode",
        false
    );
    snakecase_test!(caps_hash_sep, "FOO#BAR#BAZ", "foo_bar_baz", false);
    snakecase_test!(domain, "something.com", "something_com", false);
    snakecase_test!(
        special_leading_and_trailing,
        "$something%",
        "something",
        false
    );
    snakecase_test!(camel_case, "CStringRef", "cstring_ref", false);
    snakecase_test!(
        unicode_mixed,
        "ẞ•¶§ƒ˚foo˙∆˚¬",
        "ß_ƒ_foo",
        false
    );
    snakecase_test!(unicode_uppercase, "ẞ", "ß", false); // capitol unicode german to lowercase
    snakecase_test!(
        special_chars_long,
        "FOO:BAR$BAZ__Sample    Text___",
        "foo_bar_baz_sample_text",
        false
    );

    // ascii
    macro_rules! snakecase_ascii_test {
        ($name:ident, $input:expr, $output:expr, $b:expr) => {
            #[test]
            fn $name() {
                let results = to_snakecase_ascii($input);
                assert_eq!(results, $output);
                assert_eq!(
                    match results {
                        Cow::Borrowed(_) => true,
                        _ => false,
                    },
                    $b
                );
            }
        };
    }

    snakecase_ascii_test!(ascii_empty, "", "", true);
    snakecase_ascii_test!(ascii_equal, "sample_text", "sample_text", true);
    snakecase_ascii_test!(ascii_space, "sample text", "sample_text", false);
    snakecase_ascii_test!(ascii_dash, "sample-text", "sample_text", false);
    snakecase_ascii_test!(
        ascii_multi_underscore,
        "sample___text",
        "sample_text",
        false
    );
    snakecase_ascii_test!(
        ascii_ending_underscore,
        "sample_text_",
        "sample_text",
        false
    );
    snakecase_ascii_test!(
        ascii_ending_multi_underscore,
        "sample_text__",
        "sample_text",
        false
    );
    snakecase_ascii_test!(ascii_uppercase_sep, "sampleText", "sample_text", false);
    snakecase_ascii_test!(
        ascii_multi_uppercase,
        "inviteYourCustomersAddInvites",
        "invite_your_customers_add_invites",
        false
    );
    snakecase_ascii_test!(
        ascii_space_with_uppercase,
        "sample 2 Text",
        "sample_2_text",
        false
    );
    snakecase_ascii_test!(ascii_special_chars, "FOO:BAR$BAZ", "foo_bar_baz", false);
    snakecase_ascii_test!(ascii_caps, "samPLE text", "sam_ple_text", false);
    snakecase_ascii_test!(
        ascii_multi_spaces,
        "   sample   2    Text   ",
        "sample_2_text",
        false
    );
    snakecase_ascii_test!(
        ascii_special_with_spaces,
        "   $#$sample   2    Text   ",
        "sample_2_text",
        false
    );
    snakecase_ascii_test!(
        ascii_caps_with_space_sep,
        "SAMPLE 2 TEXT",
        "sample_2_text",
        false
    );
    snakecase_ascii_test!(
        ascii_leading_underscore_special,
        "___$$Base64Encode",
        "base64_encode",
        false
    );
    snakecase_ascii_test!(ascii_caps_hash_sep, "FOO#BAR#BAZ", "foo_bar_baz", false);
    snakecase_ascii_test!(ascii_domain, "something.com", "something_com", false);
    snakecase_ascii_test!(
        ascii_special_leading_and_trailing,
        "$something%",
        "something",
        false
    );
    snakecase_ascii_test!(ascii_camel_case, "CStringRef", "cstring_ref", false);
    snakecase_ascii_test!(
        ascii_unicode_mixed,
        "ẞ•¶§ƒ˚foo˙∆˚¬",
        "foo",
        false
    );
    snakecase_ascii_test!(ascii_unicode_uppercase, "ẞ", "", false); // capitol unicode german to lowercase
    snakecase_ascii_test!(
        ascii_special_chars_long,
        "FOO:BAR$BAZ__Sample    Text___",
        "foo_bar_baz_sample_text",
        false
    );
    snakecase_ascii_test!(ascii_digit_underscore, "5test", "5test", true);
    snakecase_ascii_test!(ascii_character_digit, "test5", "test5", true);
    snakecase_ascii_test!(ascii_uppercase_digit, "THE5r", "the5r", false);
    snakecase_ascii_test!(ascii_digit_uppercase, "5TEst", "5test", false);
}
