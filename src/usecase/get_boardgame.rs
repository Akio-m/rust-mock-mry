use crate::{domain::boardgames::Boardgame, port::boardgames::BoardgamesPort};

pub struct GetBoardgameUsecase<T: BoardgamesPort> {
    pub port: T,
}

impl<T: BoardgamesPort> GetBoardgameUsecase<T> {
    pub fn execute(&self) -> Result<Boardgame, String> {
        let boardgames = self.port.find_all().unwrap();
        Ok(boardgames.sort().get_first())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::boardgames::Boardgames, port::boardgames::*};

    impl Boardgames {
        fn mock() -> Self {
            Boardgames {
                list: vec![],
                ..Default::default()
            }
        }
    }

    impl Boardgame {
        fn mock() -> Self {
            Boardgame {
                name: "hoge".to_string(),
                price: 0,
                ..Default::default()
            }
        }
    }

    #[test]
    fn test_execute() {
        let expected = Boardgame::mock();

        let mut sorted_boardgames = Boardgames::mock();
        sorted_boardgames.mock_get_first().returns(expected.clone());

        let mut mock_boardgames = Boardgames::mock();
        mock_boardgames.mock_sort().returns(sorted_boardgames);

        let mut mock_boardgames_port = MockBoardgamesPort::default();
        mock_boardgames_port
            .mock_find_all()
            .returns(Ok(mock_boardgames));

        let target = GetBoardgameUsecase {
            port: mock_boardgames_port,
        };

        assert_eq!(target.execute(), Ok(expected));
    }
}
