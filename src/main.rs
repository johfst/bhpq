/*
mod lib;
use crate::lib::*;

fn main() -> Result<(), PriorityError> {
    let mut bhpq = BHPQ::<String>::new(8);
    bhpq.push(3, String::from("c"))?;
    bhpq.push(4, String::from("d"))?;
    bhpq.push(1, String::from("a"))?;
    bhpq.push(3, String::from("cat"))?;

    for item in bhpq {
        println!("{}", item);
    }
    Ok(())
}
*/
