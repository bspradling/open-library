use crate::arguments::Arguments;
use anyhow::Result;
use open_library::models::books::{BibliographyKey, BookIdentifier};
use open_library::models::Identifier;
use open_library::OpenLibraryClient;
use structopt::StructOpt;
use tokio;

mod arguments;

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let client = OpenLibraryClient::builder().build()?;

    let isbn = arguments.isbn().into_iter().map(|id| Identifier {
        resource: BookIdentifier::isbn_from(&id).unwrap(),
        identifier: id,
    });
    let oclc = arguments.oclc().into_iter().map(|id| Identifier {
        resource: BookIdentifier::OhioCollegeLibraryCenter,
        identifier: id,
    });
    let lccn = arguments.lccn().into_iter().map(|id| Identifier {
        resource: BookIdentifier::LibraryOfCongress,
        identifier: id,
    });
    let olid = arguments.olid().into_iter().map(|id| Identifier {
        resource: BookIdentifier::OpenLibrary,
        identifier: id,
    });

    let identifiers: Vec<BibliographyKey> = isbn
        .chain(oclc)
        .chain(lccn)
        .chain(olid)
        .map(|id| BibliographyKey::from_identifier(id).unwrap())
        .collect();

    println!("{:?}", client.books.search(&identifiers).await?);
    Ok(())
}
