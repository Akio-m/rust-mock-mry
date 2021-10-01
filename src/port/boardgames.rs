use crate::domain::boardgames::Boardgames;

use mockall::*;

#[automock]
pub trait BoardgamesPort {
    fn find_all(&self) -> Result<Boardgames, String>;
}
