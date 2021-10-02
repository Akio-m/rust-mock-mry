use crate::domain::boardgames::Boardgames;

#[mry::mry]
pub trait BoardgamesPort {
    fn find_all(&self) -> Result<Boardgames, String>;
}
