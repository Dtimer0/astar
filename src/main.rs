fn main() {
    let (x_length, y_length) = (5, 5);
    // Start of a*
}
pub enum NodeType {
    Path,
    Block,
    Start, 
    End
}
pub enum NodeState {
    Occupied,
    Unoccupied,
}
pub fn create_2D_array(x_length: i64, y_legnth: i64) -> Vec<Vec<(i64, i64, NodeType, NodeState)>> {
    todo!();
}
pub fn setStart(coordinate: (i64, i64)) -> Result<Vec<Vec<(i64, i64, NodeType, NodeState)>>, &'static str> {
    todo!();
}
pub fn setEnd(coordinate: (i64, i64)) -> Result<Vec<Vec<(i64, i64, NodeType, NodeState)>>, &'static str> {
    todo!();
}
pub fn setBlocks(coordinate: Vec<(i64, i64)>) -> Result<Vec<Vec<(i64, i64, NodeType, NodeState)>>, &'static str> {
    todo!();
}