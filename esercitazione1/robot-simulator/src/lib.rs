// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

pub struct Point{
    x:i32,
    y:i32,
}

pub struct Robot{
    dir: Direction,
    position: Point
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        let p=Point { x: x, y: y };
        Robot{ 
            dir:d,
            position:p
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let p=Point { x: self.position.x, y: self.position.y };
        Robot{
            dir:match self.dir {
            Direction::North=>Direction::East,
            Direction::East=>Direction::South,
            Direction::South=>Direction::West,
            Direction::West=>Direction::North
            },
            position:p
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let p=Point { x: self.position.x, y: self.position.y };
        Robot{
            dir:match self.dir {
            Direction::North=>Direction::West,
            Direction::West=>Direction::South,
            Direction::South=>Direction::East,
            Direction::East=>Direction::North
            },
            position:p
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let p:Point=Point{
            x: match self.dir {
                Direction::North=>self.position.x,
                Direction::West=>self.position.x-1,
                Direction::South=>self.position.x,
                Direction::East=>self.position.x+1
            },
            y:match self.dir {
                Direction::North=>self.position.y+1,
                Direction::West=>self.position.y,
                Direction::South=>self.position.y-1,
                Direction::East=>self.position.y
            }
        };
        Robot{
            dir:self.dir,
            position:p
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let char_v:Vec<char>=instructions.chars().collect();
        let p=Point { x: self.position.x, y: self.position.y };
        let mut new_robot=Robot{
            dir:self.dir,
            position:p
        };
        for c in char_v{
            new_robot=match c {
                'R'=>new_robot.turn_right(),
                'L'=>new_robot.turn_left(),
                'A'=>new_robot.advance(),
                _=>{
                    println!("Invalid command");
                    return new_robot;
                }
            };
        }
        new_robot
    }

    pub fn position(&self) -> (i32, i32) {
        return (self.position.x, self.position.y);
    }

    pub fn direction(&self) -> &Direction {
        return & self.dir;
    }
}
