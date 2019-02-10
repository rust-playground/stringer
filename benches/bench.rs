#[macro_use]
extern crate criterion;

use criterion::{Benchmark, Criterion, Throughput};
use stringer::{to_snakecase, to_snakecase_ascii};

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! snakecase_bench {
        ($name:expr,$s:expr) => {
            c.bench(
                "unicode",
                Benchmark::new($name, |b| b.iter(|| to_snakecase($s)))
                    .throughput(Throughput::Bytes($s.as_bytes().len() as u32)),
            );
        };
    }
    snakecase_bench!("snakecase_owned_simple", "sample text");
    snakecase_bench!("snakecase_borrowed_simple", "sample_text");
    snakecase_bench!(
        "snakecase_borrowed_long",
        "invite_your_customers_add_invites"
    );
    snakecase_bench!(
        "snakecase_owned_long_special_chars",
        "FOO:BAR$BAZ__Sample    Text___"
    );
    snakecase_bench!("snakecase_owned_unicode", "ẞ•¶§ƒ˚foo˙∆˚¬");
    snakecase_bench!("snakecase_borrowed_unicode", "ß_ƒ_foo");

    macro_rules! snakecase_ascii_bench {
        ($name:expr,$s:expr) => {
            c.bench(
                "ascii",
                Benchmark::new($name, |b| b.iter(|| to_snakecase_ascii($s)))
                    .throughput(Throughput::Bytes($s.as_bytes().len() as u32)),
            );
        };
    }
    snakecase_ascii_bench!("ascii_snakecase_owned_simple", "sample text");
    snakecase_ascii_bench!("ascii_snakecase_borrowed_simple", "sample_text");
    snakecase_ascii_bench!(
        "ascii_snakecase_owned_long",
        "inviteYourCustomersAddInvites"
    );
    snakecase_ascii_bench!(
        "ascii_snakecase_borrowed_long",
        "invite_your_customers_add_invites"
    );
    snakecase_ascii_bench!(
        "ascii_snakecase_owned_long_special_chars",
        "FOO:BAR$BAZ__Sample    Text___"
    );
    snakecase_ascii_bench!(
        "ascii_snakecase_owned_unicode",
        "ẞ•¶§ƒ˚foo˙∆˚¬"
    );
    snakecase_ascii_bench!("ascii_snakecase_borrowed_unicode", "ß_ƒ_foo");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
