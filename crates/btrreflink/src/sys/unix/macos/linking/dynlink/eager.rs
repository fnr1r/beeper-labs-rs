use ctor::ctor;

use crate::prelude::*;

fn initialize_clonefile() -> Result {
    super::get_clonefile()?;
    super::get_clonefileat()?;
    super::get_fclonefileat()?;
    Ok(())
}

#[ctor(unsafe)]
fn main() {
    let _ = initialize_clonefile();
}
