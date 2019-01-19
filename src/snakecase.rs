use std::borrow::Cow;

const UNDERSCORE_CHAR: char = '_';

pub fn to_snakecase<'a, S>(s: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let input = s.into();
    let mut chars = input.char_indices();
    match chars.next() {
        Some((_, c)) => {
            let mut chars = chars.fuse().peekable();
            if !c.is_alphanumeric() {
                // string needs to be modified
                let mut result: String = String::with_capacity(64); // 64 plays nich with the L2 cache in most situations
                snakecase_mod(false, &input, &mut result, &mut chars);
                return result.into();
            } else if c.is_uppercase() {
                // string needs to be modified
                let mut result: String = String::with_capacity(64); // 64 plays nich with the L2 cache in most situations
                result.push_str(&c.to_lowercase().to_string());
                // loop until finding another non-alpha or multiple underscores then add in bulk to string
                if let Some((_, c)) = chars.peek() {
                    if !c.is_uppercase() {
                        result.push(UNDERSCORE_CHAR);
                    }
                }
                snakecase_mod(false, &input, &mut result, &mut chars);
                return result.into();
            } else {
                // string is ok so far
                // return input
                while let Some((idx, c)) = chars.next() {
                    if !c.is_alphanumeric() {
                        // check for double _ with peek
                        if c == '_' {
                            if let Some((_, c2)) = chars.peek() {
                                if c2.is_lowercase() || c2.is_numeric() {
                                    // is a single underscore followed by a lowercase or digit
                                    // still no modifications needed
                                    chars.next();
                                    continue;
                                }
                            }
                        }
                        // a no go character, string needs modification
                        let mut result: String = String::with_capacity(64); // 64 plays nich with the L2 cache in most situations
                        result.push_str(&input[..idx]);
                        snakecase_mod(true, &input, &mut result, &mut chars);
                        return result.into();
                    } else if c.is_uppercase() {
                        // string needs to be modified
                        let mut result: String = String::with_capacity(64); // 64 plays nich with the L2 cache in most situations
                        result.push_str(&input[..idx]);
                        if let Some((_, c)) = chars.peek() {
                            if !c.is_uppercase() {
                                result.push(UNDERSCORE_CHAR);
                            }
                        }
                        result.push_str(&c.to_lowercase().to_string());
                        snakecase_mod(false, &input, &mut result, &mut chars);
                        return result.into();
                    }
                }
            }
            input
        }
        None => input,
    }
}

fn snakecase_mod(
    add_underscore: bool,
    input: &str,
    result: &mut String,
    chars: &mut std::iter::Peekable<std::iter::Fuse<std::str::CharIndices<'_>>>,
) {
    while let Some((start, c)) = chars.next() {
        if !c.is_alphanumeric() {
            continue;
        }

        if add_underscore {
            result.push(UNDERSCORE_CHAR);
        }

        if c.is_uppercase() {
            while let Some((end, c)) = chars.peek() {
                if !c.is_uppercase() {
                    result.push_str(&input[start..*end].to_lowercase());
                    return snakecase_mod(!c.is_lowercase(), &input, result, chars);
                }
                chars.next();
            }
            result.push_str(&input[start..].to_lowercase());
            return;
        }

        // must be lowercase
        while let Some((end, c)) = chars.peek() {
            if !c.is_lowercase() && !c.is_numeric() {
                result.push_str(&input[start..*end]);
                return snakecase_mod(true, &input, result, chars);
            }
            chars.next();
        }
        result.push_str(&input[start..]);
        return;
    }
}

pub fn to_snakecase_ascii<'a, S>(s: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let input = s.into();
    if input.is_empty() {
        return input;
    }
    let bytes = input.as_bytes();
    let l = bytes.len() - 1;
    let mut idx = 0;
    let mut b = bytes[idx];

    if !is_alphanumeric(b) {
        let mut result: String = if bytes.len() > 64 {
            String::with_capacity(bytes.len() + 7)
        } else {
            String::with_capacity(64)
        };
        // let mut result: String = String::with_capacity(bytes.len() + 7); // 64 plays nice with the L2 cache in most situations
        while idx < bytes.len() {
            b = bytes[idx];
            if !is_alphanumeric(b) {
                idx += 1;
                continue;
            }
            break;
        }
        snakecase_mod_ascii(&mut result, &bytes[idx..]);
        return result.into();
    } else if is_uppercase(b) {
        // string needs to be modified
        let mut result: String = if bytes.len() > 64 {
            String::with_capacity(bytes.len() + 7)
        } else {
            String::with_capacity(64)
        };
        result.push((b as char).to_lowercase().next().unwrap());
        // loop until finding another non-alpha or multiple underscores then add in bulk to string
        if idx < l {
            idx += 1;
            if !is_uppercase(bytes[idx]) {
                result.push(UNDERSCORE_CHAR);
            }
            snakecase_mod_ascii(&mut result, &bytes[idx..]);
        }
        return result.into();
    } else {
        // check until hitting a bad value
        while idx < bytes.len() {
            b = bytes[idx];

            if !is_alphanumeric(b) {
                // check for double _ with peek
                if b == b'_' && idx < l {
                    let b2 = bytes[idx + 1];
                    if is_lowercase(b2) || is_digit(b2) {
                        // is a single underscore followed by a lowercase or digit
                        // still no modifications needed
                        idx += 2;
                        continue;
                    }
                }
                // a no go character, string needs modification
                let mut result: String = if bytes.len() > 64 {
                    String::with_capacity(bytes.len() + 7)
                } else {
                    String::with_capacity(64)
                };
                result.push_str(&input[..idx]);
                snakecase_mod_ascii(&mut result, &bytes[idx..]);
                return result.into();
            } else if is_uppercase(b) {
                // string needs to be modified

                // although there is overhead it alows more balanced performance for both short and long input
                let mut result: String = if bytes.len() > 64 {
                    String::with_capacity(bytes.len() + 7) // if longer than 64, better to do length
                } else {
                    String::with_capacity(64) // plays nice with the L2 cache
                };
                result.push_str(&input[..idx]);
                if idx < l {
                    idx += 1;
                    let b2 = bytes[idx];
                    if !is_uppercase(b2) {
                        result.push(UNDERSCORE_CHAR);
                    }
                    result.push((b as char).to_lowercase().next().unwrap());
                    snakecase_mod_ascii(&mut result, &bytes[idx..]);
                }
                return result.into();
            }
            idx += 1;
        }
    }
    input
}

fn snakecase_mod_ascii(result: &mut String, bytes: &[u8]) {
    let mut b;
    let mut idx = 0;

    while idx < bytes.len() {
        b = bytes[idx];
        if !is_alphanumeric(b) {
            idx += 1;
            continue;
        }

        if idx > 0 {
            result.push(UNDERSCORE_CHAR);
        }

        if is_uppercase(b) {
            result.push((b as char).to_lowercase().next().unwrap());
            idx += 1;
            while idx < bytes.len() {
                b = bytes[idx];
                if is_uppercase(b) {
                    result.push((b as char).to_lowercase().next().unwrap());
                    idx += 1;
                    continue;
                }
                break;
            }
        }

        if is_lowercase(b) || is_digit(b) {
            result.push(b as char);
            idx += 1;
            while idx < bytes.len() {
                b = bytes[idx];
                if is_lowercase(b) || is_digit(b) {
                    result.push(b as char);
                    idx += 1;
                    continue;
                }
                break;
            }
        }
    }
}

#[inline]
fn is_alphabetic(b: u8) -> bool {
    match b {
        b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}

#[inline]
fn is_uppercase(b: u8) -> bool {
    match b {
        b'A'..=b'Z' => true,
        _ => false,
    }
}

#[inline]
fn is_lowercase(b: u8) -> bool {
    match b {
        b'a'..=b'z' => true,
        _ => false,
    }
}

#[inline]
fn is_digit(b: u8) -> bool {
    match b {
        b'0'..=b'9' => true,
        _ => false,
    }
}

#[inline]
fn is_alphanumeric(b: u8) -> bool {
    is_alphabetic(b) || is_digit(b)
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
    snakecase_test!(caps, "samPLE text", "sample_text", false);
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
    snakecase_ascii_test!(ascii_caps, "samPLE text", "sample_text", false);
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
}
