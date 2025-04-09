//mod _21_sum;
//mod solution_21_sum;

fn maina() {
    // TODO einfaches hallo welt
}

// TODO einfacher test

enum Error {
    InvalidInput,
    NoName,
}
fn parse() -> Result<(usize, String), Error> {
    let mut input = "15 Bear".split(' ');
    // Need to pull the number and parse it.
    let number: usize = input
        .next()
        // Process Option<&'static str> to Result<int>
        .and_then(|x| x.parse().ok())
        .ok_or(Error::InvalidInput)?;
    // The next token is our animal.
    let animal = input.next().ok_or(Error::NoName)?;
    // Ouput `number` times.
    Ok((number, animal.to_string())).clone()
}

struct ParseResult<'a> {
    number: usize,
    animal: &'a str,
}

fn decode<'a>(input: &'a str) -> Result<ParseResult<'a>, Error> {
    todo!()
}

fn main() {
    let mut input = "15 Bear";
    let result = decode(input);

    // input kann nicht geändert werden

    println!("result: {:?}", result);
}