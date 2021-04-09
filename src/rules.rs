/// This is the default rule set for Conway's game of life, but this can be replaced by any
/// arbitrary function on the (current cell, 8 neighbors) tuple returning a bool
pub fn default_rule(tup: (bool, [bool; 8])) -> bool {
    let (cell, arr) = tup;
    let neighbor_count = arr
        .iter()
        .fold(0, |acc, item| if *item { acc + 1 } else { acc });
    if neighbor_count == 3 {
        return true;
    }
    if cell && neighbor_count == 2 {
        return true;
    }
    return false;
}
