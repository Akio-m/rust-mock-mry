#[mry::mry]
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Boardgames {
    pub list: Vec<Boardgame>,
}

#[mry::mry]
impl Boardgames {
    pub fn sort(&self) -> Self {
        todo!()
    }

    pub fn get_first(&self) -> Boardgame {
        todo!()
    }
}

#[mry::mry]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Boardgame {
    pub name: String,
    pub price: i32,
}
