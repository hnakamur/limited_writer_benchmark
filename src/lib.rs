use std::fmt::{self, Write};

pub struct LimitedWriter<'a, T: ?Sized + 'a> {
    inner: &'a mut T,
    limit: usize,
    n: usize,
}

impl<'a, T> LimitedWriter<'a, T> {
    pub fn new(inner: &'a mut T, limit: usize) -> Self {
        Self { inner, limit, n: 0 }
    }

    pub fn num_bytes_would_written(&self) -> usize {
        self.n
    }
}

impl<T: Write + ?Sized> fmt::Write for LimitedWriter<'_, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.n + s.len() <= self.limit {
            self.inner.write_str(&s)?;
        } else {
            let mut i = 0usize;
            while i < s.len() {
                let len = len_utf8_at(&s, i);
                if self.n + i + len > self.limit {
                    break;
                }
                i += len;
            }
            self.inner.write_str(&s[0..i])?;
        }
        self.n += s.len();
        Ok(())
    }
}

fn len_utf8_at(s: &str, i: usize) -> usize {
    let b = s.as_bytes()[i];
    if b & 0b1000_0000 == 0 {
        1
    } else if b & 0b1110_0000u8 == 0b1100_0000u8 {
        2
    } else if b & 0b1111_0000u8 == 0b1110_0000u8 {
        3
    } else if b & 0b1111_1000u8 == 0b1111_0000u8 {
        4
    } else {
        panic!("index not at char boundary: {}", i);
    }
}

pub struct LimitedWriter2<'a, T: ?Sized + 'a> {
    inner: &'a mut T,
    limit: usize,
    n: usize,
}

impl<'a, T> LimitedWriter2<'a, T> {
    pub fn new(inner: &'a mut T, limit: usize) -> Self {
        Self { inner, limit, n: 0 }
    }

    pub fn num_bytes_would_written(&self) -> usize {
        self.n
    }
}

impl<T: Write + ?Sized> fmt::Write for LimitedWriter2<'_, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.n + s.len() <= self.limit {
            self.inner.write_str(&s)?;
        } else {
            let mut char_indices = s.char_indices();
            let mut i = 0;
            while let Some((j, _)) = char_indices.next() {
                if self.n + j > self.limit {
                    break;
                }
                i = j;
            }
            self.inner.write_str(&s[0..i])?;
        }
        self.n += s.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_utf8_at() {
        let s = "\u{0024}\u{00A2}\u{20AC}\u{10348}";
        assert_eq!(len_utf8_at(s, 0), 1);
        assert_eq!(len_utf8_at(s, 1), 2);
        assert_eq!(len_utf8_at(s, 3), 3);
        assert_eq!(len_utf8_at(s, 6), 4);
    }

    #[test]
    fn test_limited_writer() {
        let s = "\u{0024}\u{00A2}\u{20AC}\u{10348}";
        #[derive(Debug)]
        struct TestCase<'a> {
            limit: usize,
            want: &'a str,
        }
        const CASES: [TestCase; 8] = [
            TestCase { limit: 0, want: "" },
            TestCase {
                limit: 1,
                want: "\u{0024}",
            },
            TestCase {
                limit: 2,
                want: "\u{0024}",
            },
            TestCase {
                limit: 3,
                want: "\u{0024}\u{00A2}",
            },
            TestCase {
                limit: 5,
                want: "\u{0024}\u{00A2}",
            },
            TestCase {
                limit: 6,
                want: "\u{0024}\u{00A2}\u{20AC}",
            },
            TestCase {
                limit: 9,
                want: "\u{0024}\u{00A2}\u{20AC}",
            },
            TestCase {
                limit: 10,
                want: "\u{0024}\u{00A2}\u{20AC}\u{10348}",
            },
        ];
        for (i, c) in CASES.iter().enumerate() {
            let mut buf = String::new();
            let mut w = LimitedWriter::new(&mut buf, c.limit);
            write!(&mut w, "{}", s).unwrap();
            assert_eq!(buf, c.want, "i={}, buf={}, c={:?}", i, buf, c);
        }
    }

    #[test]
    fn test_limited_writer2() {
        let s = "\u{0024}\u{00A2}\u{20AC}\u{10348}";
        #[derive(Debug)]
        struct TestCase<'a> {
            limit: usize,
            want: &'a str,
        }
        const CASES: [TestCase; 8] = [
            TestCase { limit: 0, want: "" },
            TestCase {
                limit: 1,
                want: "\u{0024}",
            },
            TestCase {
                limit: 2,
                want: "\u{0024}",
            },
            TestCase {
                limit: 3,
                want: "\u{0024}\u{00A2}",
            },
            TestCase {
                limit: 5,
                want: "\u{0024}\u{00A2}",
            },
            TestCase {
                limit: 6,
                want: "\u{0024}\u{00A2}\u{20AC}",
            },
            TestCase {
                limit: 9,
                want: "\u{0024}\u{00A2}\u{20AC}",
            },
            TestCase {
                limit: 10,
                want: "\u{0024}\u{00A2}\u{20AC}\u{10348}",
            },
        ];
        for (i, c) in CASES.iter().enumerate() {
            if i != 7 {
                continue;
            }
            let mut buf = String::new();
            let mut w = LimitedWriter2::new(&mut buf, c.limit);
            write!(&mut w, "{}", s).unwrap();
            assert_eq!(buf, c.want, "i={}, buf={}, c={:?}", i, buf, c);
        }
    }
}
