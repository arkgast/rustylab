//! Runner for S1-T4: `scripts/run.sh S1-T4`. Add your own `show!` lines freely.
use tasks::s01::t04_borrowing::*;
use tasks::{lab::kata, show};

fn main() {
    kata("largest / sum_evens / dedup_sorted_len", || {
        show!(largest(&[1, 9, -4, 9, 2]));
        show!(largest(&[]));
        show!(sum_evens(&[1, 2, 3, 4, -6, 10]));
        show!(dedup_sorted_len(&[1, 1, 2, 3, 3, 3]));
    });
    kata("rotate_left", || {
        let mut v = [1, 2, 3, 4, 5];
        rotate_left(&mut v, 2);
        show!(v);
    });
    kata("first_word / count_words", || {
        show!(first_word("  hello world"));
        show!(first_word("ñandú rápido"));
        show!(count_words(" two  words\n"));
    });
    kata("longest_line", || {
        show!(longest_line("short\na much longer line\nmid"));
        show!(longest_line("abcd\n🦀🦀🦀🦀🦀"));
    });
    kata("split_header_body", || {
        show!(split_header_body("Host: x\nA: b\n\nhello"));
        show!(split_header_body("no blank line"));
    });
    kata("truncate_chars", || {
        show!(truncate_chars("héllo", 2));
        show!(truncate_chars("🦀rust", 1));
        show!(truncate_chars("hi", 10));
    });
    kata("capitalize_words", || {
        show!(capitalize_words("hello  rust world"));
        show!(capitalize_words("éclair über"));
    });
}
