mod storage;
mod vocab;
mod tui;

use crate::storage::AnyResult;


fn main() ->  AnyResult<()> {
    tui::run_tui()?;
    Ok(())
}
