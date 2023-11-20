use std::ops::Index;

fn main() {
    let start = (4, 0);
    let end = (0, 0);
    let mut current = (0, 0);
    let mut frontier_set : Vec<(usize, usize)> = Vec::new();
    let mut unexplored_set : Vec<(usize, usize)> = Vec::new();
    let mut explored_set : Vec<(usize, usize)> = Vec::new();
    let (x_length, y_length) = (5, 5);
    let mut map: Vec<Vec<NodeType>> = create_2d_array(x_length, y_length);
    map.set_start(start).unwrap();
    map.set_end(end).unwrap();
    map.set_blocks(Vec::from([(1, 0), (1, 1), (1, 3), (1, 4)])).unwrap();
    map.iter().for_each(|it| {println!(); it.iter().for_each(|x| {print!("[{:#?}] ", x)})});
    for i in 0..x_length {
        for j in 0..y_length {
            if map[i as usize][j as usize] == NodeType::Start {
                current = (i as usize, j as usize);
            }
            else if map[i as usize][j as usize] == NodeType::Path {
                unexplored_set.push((i as usize, j as usize));
            }
        }
    }
    println!();
    let mut current_lowest_f_cost = -1;
    let mut lowest_f_costs: Vec<(i64, i64)> = Vec::new();
    let mut neighbour_list: Vec<(i64, i64)> = Vec::new();
    // Start of algorithm
    frontier_set.push(current);
    loop {
        lowest_f_costs.clear();
        for neighbours in frontier_set.clone() {
            neighbour_list.append(&mut get_neighbours(map.clone(), (neighbours.0 as i64, neighbours.1 as i64)));
        }
        for neighbour in neighbour_list.clone() {
            if current_lowest_f_cost == -1 {
                current_lowest_f_cost = get_f_cost(neighbour, start, end);
                lowest_f_costs.push(neighbour);
            }
            else if get_f_cost(neighbour, start, end) < current_lowest_f_cost {
                current_lowest_f_cost = get_f_cost(neighbour, start, end);
                lowest_f_costs.clear();
                lowest_f_costs.push(neighbour);
            }
            else if get_f_cost(neighbour, start, end) == current_lowest_f_cost {
                lowest_f_costs.push(neighbour);
            }
            println!("{:?}, F_cost: {}, H_cost: {}", neighbour, get_f_cost(neighbour, start, end), get_h_cost(neighbour, end));
            frontier_set.push((neighbour.0 as usize, neighbour.1 as usize));
        }
        let mut current_lowest_h_cost = -1;
        lowest_f_costs.iter().for_each(|x| if explored_set.contains(&(x.clone().0 as usize, x.clone().1 as usize)) {lowest_f_costs.remove(lowest_f_costs.iter().position(|&y| y == *x).unwrap());});
        for lowest_f_cost in lowest_f_costs.clone() {
            if lowest_f_cost == end {
                println!("Found end");
                break;
            }
            else {
                if current_lowest_h_cost == -1 {
                    current_lowest_h_cost = get_h_cost(lowest_f_cost, end);
                    current = (lowest_f_cost.0 as usize, lowest_f_cost.1 as usize);
                }
                else if get_h_cost(lowest_f_cost, end) < current_lowest_h_cost {
                    current_lowest_h_cost = get_h_cost(lowest_f_cost, end);
                    current = (lowest_f_cost.0 as usize, lowest_f_cost.1 as usize);
                }
            }
        }
        let index = unexplored_set.iter().position(|&x| x == current).unwrap();
        unexplored_set.remove(index);
        let index = frontier_set.iter().position(|&x| x == current).unwrap();
        frontier_set.remove(index);
        explored_set.push(current);
        println!("{}, {:?}", current_lowest_f_cost, lowest_f_costs);
    }   
}

trait X {
    fn set_start(&mut self, start: (i64, i64)) -> Result<(), &'static str>;
    fn set_end(&mut self, end: (i64, i64)) -> Result<(), &'static str>;
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
    fn set_start(&mut self, start: (i64, i64)) -> Result<(), &'static str> {
        if self[start.0 as usize][start.1 as usize] == NodeType::End {
            return Err("Start and end cannot be the same");
        }
        else if self[start.0 as usize][start.1 as usize] == NodeType::Block {
            return Err("Start cannot be a block");
        }
        else if !in_range(start.0 as usize, start.1 as usize, self.len(), self[0].len()) {
            return Err("Out of range");
        }
        self[start.0 as usize][start.1 as usize] = NodeType::Start;
        Ok(())
    }
    fn set_end(&mut self, end: (i64, i64)) -> Result<(), &'static str> {
        if self[end.0 as usize][end.1 as usize] == NodeType::Start {
            return Err("Start and end cannot be the same");
        }
        else if self[end.0 as usize][end.1 as usize] == NodeType::Block {
            return Err("End cannot be a block");
        }
        else if !in_range(end.0 as usize, end.1 as usize, self.len(), self[0].len()) {
            return Err("Out of range");
        }
        self[end.0 as usize][end.1 as usize] = NodeType::End;
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

pub fn get_neighbours(map: Vec<Vec<NodeType>>, node: (i64, i64)) -> Vec<(i64, i64)>{
    let directions: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let (x_dim, y_dim) = (map.len() as i64, map[0].len() as i64);
    let mut neighbours: Vec<(i64, i64)> = Vec::new();
    for &(dr, dc) in &directions {
        let new_row = node.0 as isize + dr as isize;
        let new_col = node.1 as isize + dc as isize;
        // Check if the new position is within bounds
        if new_row >= 0 && new_row < x_dim as isize && new_col >= 0 && new_col < y_dim as isize && map[new_row as usize][new_col as usize] != NodeType::Block{
            neighbours.push((new_row as i64, new_col as i64));
        }
    }
    neighbours
}

pub fn get_h_cost(node: (i64, i64), end: (i64, i64)) -> i64 {
    let (x1, y1) = node;
    let (x2, y2) = end;
    (x1 - x2).abs() + (y1 - y2).abs()
}
pub fn get_g_cost(node: (i64, i64), start: (i64, i64)) -> i64 {
    let (x1, y1) = node;
    let (x2, y2) = start;
    (x1 - x2).abs() + (y1 - y2).abs() 
}
pub fn get_f_cost(node: (i64, i64), start: (i64, i64), end: (i64, i64)) -> i64 {
    get_g_cost(node, start) + get_h_cost(node, end)
}

