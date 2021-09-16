use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Arguments", about = "Retrieve book information by identifier")]
pub struct Arguments {
    #[structopt(
        global = true,
        long = "isbn",
        help = "The International Standard Book Number of the desired book"
    )]
    isbn: Vec<String>,

    #[structopt(
        global = true,
        long = "oclc",
        help = "The Ohio College Library Center identifier of the desired book"
    )]
    oclc: Vec<String>,

    #[structopt(
        global = true,
        long = "lccn",
        help = "The Library of Congress Control Number of the desired book"
    )]
    lccn: Vec<String>,

    #[structopt(
        global = true,
        long = "olid",
        help = "The Open Library identifier of the desired book"
    )]
    olid: Vec<String>,
}

impl Arguments {
    pub fn isbn(&self) -> Vec<String> {
        self.isbn.clone()
    }

    pub fn lccn(&self) -> Vec<String> {
        self.lccn.clone()
    }

    pub fn oclc(&self) -> Vec<String> {
        self.oclc.clone()
    }

    pub fn olid(&self) -> Vec<String> {
        self.olid.clone()
    }
}
