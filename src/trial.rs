trait Strategy {
    fn best_word(&self) -> &'static str;
}

struct TestPuzzle<T>
where
    T: Strategy,
{
    n_turns: u8,
    answer: &'static str,
    strategy: T,
}

impl<T> TestPuzzle<T>
where
    T: Strategy,
{
    pub fn new(n_turns: u8, answer: &'static str, strategy: T) -> Self {
        TestPuzzle {
            n_turns,
            answer,
            strategy,
        }
    }
}
