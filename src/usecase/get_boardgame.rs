use crate::domain::boardgames::Boardgames;
use crate::port::boardgames::BoardgamesPort;

pub struct GetBoardgameUsecase<T: BoardgamesPort> {
    pub port: T,
}

impl<T: BoardgamesPort> GetBoardgameUsecase<T> {
    pub fn execute(&self) -> Result<Boardgames, String> {
        let boardgames = self.port.find_all().unwrap();
        Ok(boardgames.sort())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::boardgames::*;

    impl Boardgames {
        fn mock() -> Self {
            Boardgames {
                list: vec![],
                ..Default::default()
            }
        }
    }

    #[test]
    fn test_execute() {
        let expected = Boardgames::mock();

        let mut mock_boardgames = Boardgames::mock();
        mock_boardgames.mock_sort().returns(expected.clone());

        let mut mock_boardgames_port = MockBoardgamesPort::new();
        mock_boardgames_port
            .expect_find_all()
            .return_const(Ok(mock_boardgames));

        let target = GetBoardgameUsecase {
            port: mock_boardgames_port,
        };

        assert_eq!(target.execute(), Ok(expected));
    }
}
