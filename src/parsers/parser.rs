pub trait Parser {
    fn next(&mut self, c: char);
}
