extern crate sale_db;
use failure::Error;
use sale_db::Conn;

fn main() -> Result<(), Error> {
    let conn = Conn::new()?;
    let r = conn.put_currency("PEN", "Soles from Peru")?;
    println!("Added currency: {:?}", r);
    Ok(())
}