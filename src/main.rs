use nom::{
    branch::permutation,
    bytes::complete::{tag, tag_no_case},
    character::complete::{alpha1, digit1},
    combinator::{map_res, opt},
    error::{ErrorKind, ParseError},
    sequence::delimited,
    IResult,
};
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
    language: &'a str,
}

fn name_parser(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("my name is ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, name))
}
fn age_parser(input: &str) -> IResult<&str, u8> {
    let (input, _) = opt(tag_no_case(" and"))(input)?;
    let (input, _) = tag_no_case(" i am ")(input)?;
    let (input, age) = map_res(digit1, |s: &str| s.parse::<u8>())(input)?;

    //consume the rest and return smth even if empty str since it could be the rest
    let (input, _) = tag_no_case(" years old")(input)?;

    println!("input left after age parser {input}");
    Ok((input, age))
}
fn language_parser(input: &str) -> IResult<&str, &str> {
    let (input, _) = opt(tag_no_case(" and"))(input)?;
    let (input, _) = tag_no_case(" i like ")(input)?;
    let (input, language) = alpha1(input)?;
    println!("input left after language parser {input}");
    Ok((input, language))
}

fn parse_person<'a>(input: &'a str) -> IResult<&'a str, Person> {
    // //returns what remains after 'is' and also the tag we gave it
    // //which is 'Hello, my name is'. We don't need the tag
    // let (input, _) = tag("Hello, my name is ")(input)?;
    // //assigns ' and I am 30 years old' to input and 'Farah' to name,
    // // alpha1 takes alphabets until a non alphabet
    // //like space or number
    // let (input, name) = alpha1(input)?;
    // //leaves '30 years old.' in input
    // let (input, _) = tag(" and I am ")(input)?;
    // // map_res takes a parser and closure, digit1 will collect
    // // numbers in string form then the closure can convert it to u8
    // let (input, age) = map_res(digit1, |s: &str| s.parse::<u8>())(input)?;

    // //input should have ' years old.' remaining

    let (input, _) = tag("Hello, ")(input)?;
    let (input, (name, age, language)) =
        permutation((name_parser, age_parser, language_parser))(input)?;

    Ok((
        input,
        Person {
            name,
            age,
            language,
        },
    ))
}
fn main() {
    // Input like so:
    //"Hello, my name is Farah and I am 30 years old."
    dbg!(parse_person(
        "Hello, my name is Farah and I am 30 years old and I like rust."
    ));
    dbg!(parse_person(
        "Hello, my name is Finn and I like python and I am 40 years old."
    ));

    dbg!(parser("begining middle end."));
}

fn parser(input: &str) -> IResult<&str, &str> {
    delimited(tag("begining "), tag("middle"), tag(" end."))(input)
}
