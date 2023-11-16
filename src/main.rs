fn main() {
    let (start_x, start_y) = (4, 0);
    let (end_x, end_y) = (0, 0);
    let mut open_set : Vec<(usize, usize)> = Vec::new();
    let mut closed_set : Vec<(usize, usize)> = Vec::new();
    let (x_length, y_length) = (5, 5);
    let mut map: Vec<Vec<NodeType>> = create_2d_array(x_length, y_length);
    map.set_start(start_x, start_y).unwrap();
    map.set_end(end_x, end_y).unwrap();
    map.set_blocks(Vec::from([(1, 0), (1, 1), (1, 3), (1, 4)])).unwrap();
    map.iter().for_each(|it| {println!(); it.iter().for_each(|x| {print!("[{:#?}] ", x)})});
    open_set.push((start_x, start_y));
    println!("Neighbours: {:?}", get_neighbours(map, 0, 3));
    while open_set.len() > 0 {
        break;
    }
    println!("Closed set: {:?}", closed_set);
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
    End,
}
pub fn create_2d_array(x_length: i64, y_length: i64) -> Vec<Vec<NodeType>> {
    vec![vec![NodeType::Path; x_length as usize]; y_length as usize]
}
impl X for Vec<Vec<NodeType>> {
    fn set_start(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if self[x][y] == NodeType::End {
            return Err("Start and end cannot be the same");
        }
        else if self[x][y] == NodeType::Block {
            return Err("Start cannot be a block");
        }
        else if !in_range(x, y, self.len(), self[0].len()) {
            return Err("Out of range");
        }
        self[x][y] = NodeType::Start;
        Ok(())
    }
    fn set_end(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if self[x][y] == NodeType::Start {
            return Err("Start and end cannot be the same");
        }
        else if self[x][y] == NodeType::Block {
            return Err("End cannot be a block");
        }
        else if !in_range(x, y, self.len(), self[0].len()) {
            return Err("Out of range");
        }
        self[x][y] = NodeType::End;
        Ok(())
    }
    fn set_blocks(&mut self, coordinates: Vec<(usize, usize)>) ->Result<(), &'static str> {
        for coord in coordinates {
            if self[coord.0][coord.1] == NodeType::Start || self[coord.0][coord.1] == NodeType::End {
                return Err("Block cannot be start or end");
            }
            else if !in_range(coord.0, coord.1, self.len(), self[0].len()) {
                return Err("Out of range");
            }
            self[coord.0][coord.1] = NodeType::Block;
            
        }
        Ok(())
    }
}   

pub fn in_range(x: usize, y: usize, x_length: usize, y_length: usize) -> bool {
    x < x_length && y < y_length 
}

pub fn get_neighbours(map: Vec<Vec<NodeType>>, x: i64, y: i64) -> Vec<(i64, i64)>{
    let directions: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let (x_dim, y_dim) = (map.len() as i64, map[0].len() as i64);
    let mut neighbours: Vec<(i64, i64)> = Vec::new();
    for &(dr, dc) in &directions {
        let new_row = x as isize + dr as isize;
        let new_col = y as isize + dc as isize;
        // Check if the new position is within bounds
        if new_row >= 0 && new_row < x_dim as isize && new_col >= 0 && new_col < y_dim as isize && map[new_row as usize][new_col as usize] != NodeType::Block{
            neighbours.push((new_row as i64, new_col as i64));
        }
    }
    neighbours
}

pub fn get_h_cost(node: (i64, i64), end: (i64, i64)) -> f64 {
    let (x1, y1) = node;
    let (x2, y2) = end;
    f64::sqrt(((x1 - x2).abs() + (y1 - y2).abs()) as f64)
}