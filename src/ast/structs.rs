#[derive(Debug)]
pub struct LineEnd<'a> {
    pub ws: &'a str,
    pub comment: &'a str,
    pub nl: &'a str,
}

