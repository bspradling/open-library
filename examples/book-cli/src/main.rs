use crate::arguments::Arguments;
use anyhow::Result;
use open_library::models::books::BibliographyKey::{ISBN, LCCN, OCLC, OLID};
use open_library::models::books::{BibliographyKey, BookIdentifierKey};
use open_library::OpenLibraryClient;
use structopt::StructOpt;
use tokio;

mod arguments;

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let client = OpenLibraryClient::builder().build()?;

    let isbn = arguments.isbn().into_iter().map(ISBN);
    let oclc = arguments.oclc().into_iter().map(OCLC);
    let lccn = arguments.lccn().into_iter().map(LCCN);
    let olid = arguments.olid().into_iter().map(OLID);

    let identifiers: Vec<BibliographyKey> = isbn.chain(oclc).chain(lccn).chain(olid).collect();

    println!("{:?}", client.books.search(&identifiers).await?);
    Ok(())
}
