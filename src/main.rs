fn main() {
    let (x_length, y_length) = (5, 5);
    let mut map: Vec<Vec<NodeType>> = create_2d_array(x_length, y_length);
    map.set_start(4, 0).unwrap();
    map.iter().for_each(|it| {println!(); it.iter().for_each(|x| {print!("{:#?}, ", x)})});
}
trait X {
    fn set_start(&mut self, x: usize, y: usize) -> Result<(), &'static str>;
    fn set_end(&mut self, x: usize, y: usize) -> Result<(), &'static str>;
    fn set_blocks(&mut self, coordinates: Vec<(usize, usize)>) ->Result<(), &'static str>;
}
#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Path,
    Block,
    Start, 
    End
}
pub fn create_2d_array(x_length: i64, y_length: i64) -> Vec<Vec<NodeType>> {
    vec![vec![NodeType::Path; x_length as usize]; y_length as usize]
}
impl X for Vec<Vec<NodeType>> {
    fn set_start(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if !in_range(x, y, self.len(), self[0].len()) {
            return Err("Out of range");
        }
        self[x][y] = NodeType::Start;
        Ok(())
    }
    fn set_end(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        todo!();
    }
    fn set_blocks(&mut self, coordinates: Vec<(usize, usize)>) ->Result<(), &'static str> {
        todo!();
    }
}   

pub fn in_range(x: usize, y: usize, x_length: usize, y_length: usize) -> bool {
    x < x_length && y < y_length 
}