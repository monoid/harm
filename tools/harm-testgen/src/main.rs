use clap::Parser as ClapParser;
use itertools::Itertools;
use nom::{
    IResult, Parser as _,
    branch::alt,
    bytes::complete::tag,
    character::complete::none_of,
    combinator::{cut, eof, map},
    multi::{many, separated_list0},
    sequence::{preceded, terminated},
};

#[derive(ClapParser)]
struct Args {
    pattern: String,
}

fn main() {
    let args = Args::parse();
    for inst in variants(&args.pattern) {
        println!("\t{inst}");
    }
}

fn variants(pattern: &str) -> impl Iterator<Item = String> {
    nom::combinator::all_consuming(parse)
        .parse(pattern)
        .unwrap()
        .1
}

struct IterClone<Item>(Box<dyn BoxCloneIter<Item = Item>>);

impl<Item> IterClone<Item> {
    pub fn new<I: Iterator<Item = Item> + Clone + 'static>(it: I) -> Self {
        Self(Box::new(it) as _)
    }
}

impl<Item> Clone for IterClone<Item> {
    fn clone(&self) -> Self {
        Self(self.0.box_clone())
    }
}

impl<Item> Iterator for IterClone<Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

trait BoxCloneIter: Iterator {
    fn box_clone(&self) -> Box<dyn BoxCloneIter<Item = <Self as Iterator>::Item>>;
}

impl<T> BoxCloneIter for T
where
    T: Iterator + Clone + 'static,
{
    fn box_clone(&self) -> Box<dyn BoxCloneIter<Item = <Self as Iterator>::Item>> {
        Box::new(Clone::clone(self)) as _
    }
}

fn parse(inp: &str) -> IResult<&str, IterClone<String>> {
    alt((
        map(eof, |_| IterClone::new(std::iter::once(String::new()))),
        map(many(1.., parse_item), |items: Vec<_>| {
            IterClone::new(
                items
                    .into_iter()
                    .multi_cartesian_product()
                    .map(|prod| format!("{}", prod.into_iter().format(""))),
            )
        }),
    ))
    .parse(inp)
}

fn parse_item(inp: &str) -> IResult<&str, IterClone<String>> {
    alt((
        preceded(tag("("), cut(terminated(parse_alt, tag(")")))),
        map(many(1.., none_of("(|)")), |c: String| {
            IterClone::new(std::iter::once(c))
        }),
    ))
    .parse(inp)
}

fn parse_alt(inp: &str) -> IResult<&str, IterClone<String>> {
    map(separated_list0(tag("|"), parse), |list| {
        IterClone::new(list.into_iter().flatten())
    })
    .parse(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_item_char() {
        let single = parse_item("a").unwrap().1.collect_vec();
        assert_eq!(single, &["a"])
    }

    #[test]
    fn test_parse_char() {
        let single = parse("a").unwrap().1.collect_vec();
        assert_eq!(single, &["a"])
    }

    #[test]
    fn test_parse_char_incomplete() {
        let (rest, single) = parse("a)").unwrap();
        assert_eq!(rest, ")");
        assert_eq!(single.collect_vec(), &["a"])
    }

    #[test]
    fn test_parse_alt() {
        let alt = parse_alt("a|b").unwrap().1.collect_vec();
        assert_eq!(alt, &["a", "b"]);
    }

    #[test]
    fn test_empty() {
        let empty = variants("").collect_vec();
        assert_eq!(empty, &[""]);
    }

    #[test]
    fn test_simple() {
        let simple = variants("simple").collect_vec();
        assert_eq!(simple, &["simple"]);
    }

    #[test]
    fn test_words() {
        let var = variants("a b").collect_vec();
        assert_eq!(var, &["a b"]);
    }

    #[test]
    fn test_patterns() {
        let var = variants("(a|b) (c|d)").collect_vec();
        assert_eq!(var, &["a c", "a d", "b c", "b d"]);
    }

    #[test]
    fn test_patterns_nested1() {
        let var = variants("(a|b(c|d))").collect_vec();
        assert_eq!(var, &["a", "bc", "bd"]);
    }

    #[test]
    fn test_patterns_nested2() {
        let var = variants("(a|(c|d)b)").collect_vec();
        assert_eq!(var, &["a", "cb", "db"]);
    }

    #[test]
    fn test_patterns_nested3() {
        let var = variants("(a|b(c|d)e)").collect_vec();
        assert_eq!(var, &["a", "bce", "bde"]);
    }
}
