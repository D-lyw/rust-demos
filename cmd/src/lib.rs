mod cli;
mod process;
mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use utils::*;
pub use text::*;
pub use passgen::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CommandExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
