use std::io::stdin;

pub struct Finder<'a, 'b> {
    substrings: &'a [&'b str],
    tracking_candidates: Vec<(&'b str, &'b str)>,
}

impl<'a, 'b> Finder<'a, 'b> {
    pub fn new(substrings: &'a [&'b str]) -> Self {
        Self {
            substrings,
            tracking_candidates: vec![],
        }
    }

    pub fn next_char(&mut self, char: char) -> Option<&'b str> {
        // lazy version which could be optimized (by checking new candidates after the retain)
        for candidate in self.substrings.iter() {
            self.tracking_candidates.push((candidate, candidate));
        }
        self.tracking_candidates
            .retain_mut(|(_, ref mut remainder)| {
                if dbg!(char)
                    == dbg!(remainder
                        .chars()
                        .next()
                        .expect("We exit before the str slice becomes empty!"))
                {
                    *remainder = &remainder[1..];
                    true
                } else {
                    false
                }
            });

        for (substring, remainder) in &self.tracking_candidates {
            if remainder.is_empty() {
                return Some(substring);
            }
        }

        None
    }
}

fn find_first<'a>(target: &'_ str, substrings: &'_ [&'a str]) -> Option<&'a str> {
    let mut finder = Finder::new(substrings);
    for char in target.chars() {
        if let result @ Some(_) = finder.next_char(char) {
            return result;
        }
    }
    None
}

fn main() {
    let mut total: usize = 0;
    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut current = 0;
        for char in line.chars() {
            if let Some(value) = char.to_digit(10) {
                current += value * 10;
                break;
            }
        }

        for char in line.chars().rev() {
            if let Some(value) = char.to_digit(10) {
                current += value;
                break;
            }
        }

        println!("Read \"{line}\", adding {current}");
        total += current as usize;
    }
    println!("Total value: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_substrings() {
        let target = "onetwothree";

        assert!(None == find_first(target, &[]));
    }

    #[test]
    fn one_substring() {
        let target = "onetwothree";

        assert!(Some("one") == find_first(target, &["one"]));
    }

    #[test]
    fn two_matches() {
        let target = "twone";

        assert!(Some("two") == find_first(target, &["one", "two"]));
    }
}
