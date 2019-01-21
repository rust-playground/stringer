#![feature(test)]

extern crate test;

use stringer::{to_snakecase, to_snakecase_ascii};

macro_rules! snakecase_bench {
    ($name:ident, $s:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            let s = $s;
            b.bytes = (s.bytes().count()) as u64;
            b.iter(|| to_snakecase(s));
        }
    };
}

snakecase_bench!(snakecase_owned_simple, "sample text");
snakecase_bench!(snakecase_borrowed_simple, "sample_text");
snakecase_bench!(snakecase_owned_long, "inviteYourCustomersAddInvites");
snakecase_bench!(snakecase_borrowed_long, "invite_your_customers_add_invites");
snakecase_bench!(
    snakecase_owned_long_special_chars,
    "FOO:BAR$BAZ__Sample    Text___"
);
snakecase_bench!(snakecase_owned_unicode, "ẞ•¶§ƒ˚foo˙∆˚¬");
snakecase_bench!(snakecase_borrowed_unicode, "ß_ƒ_foo");

macro_rules! snakecase_ascii_bench {
    ($name:ident, $s:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            let s = $s;
            b.bytes = (s.bytes().count()) as u64;
            b.iter(|| to_snakecase_ascii(s));
        }
    };
}

snakecase_ascii_bench!(ascii_snakecase_owned_simple, "sample text");
snakecase_ascii_bench!(ascii_snakecase_borrowed_simple, "sample_text");
snakecase_ascii_bench!(ascii_snakecase_owned_long, "inviteYourCustomersAddInvites");
snakecase_ascii_bench!(
    ascii_snakecase_borrowed_long,
    "invite_your_customers_add_invites"
);
snakecase_ascii_bench!(
    ascii_snakecase_owned_long_special_chars,
    "FOO:BAR$BAZ__Sample    Text___"
);
snakecase_ascii_bench!(ascii_snakecase_owned_unicode, "ẞ•¶§ƒ˚foo˙∆˚¬");
snakecase_ascii_bench!(ascii_snakecase_borrowed_unicode, "ß_ƒ_foo");
