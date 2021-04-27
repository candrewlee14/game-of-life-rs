pub mod game;
pub mod rules;

#[cfg(test)]
mod tests {
    use crate::game::Grid;
    use crate::rules::default_rule;
    #[test]
    fn test_flip_flop_wrap_sequential(){
        let (width, height) = (100, 100);
        let mut game = Grid::new_empty(width, height);
        game.set_cell(0, 0, true);
        game.set_cell(1, 0, true);
        game.set_cell(99, 0, true);
        game.propogate(default_rule);
        assert!(game.get_cell(0, 0) && game.get_cell(0, 1) && game.get_cell(0, 99));
    }
    #[test]
    fn test_flip_flop_wrap_parallel(){
        let (width, height) = (100, 100);
        let mut game = Grid::new_empty(width, height);
        game.set_cell(0, 0, true);
        game.set_cell(1, 0, true);
        game.set_cell(99, 0, true);
        game.propogate_par(default_rule);
        assert!(game.get_cell(0, 0) && game.get_cell(0, 1) && game.get_cell(0, 99));
    }
}
