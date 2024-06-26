use crate::auth::models::ListUser;
use csv::Writer;
use std::error::Error;

pub async fn write_to_csv(data: Vec<ListUser>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(vec![]);

    for pat in data {
        wtr.serialize(pat)?;
    }

    wtr.flush()?;

    Ok(())
}
