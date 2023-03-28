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

use clap::Parser;
/// Move your robot
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
///xposition
    #[arg(short, long)]
    xpos: String,
///yposition
    #[arg(short, long)]
    ypos: String,
///direction
    #[arg(short, long)]
    dir: String,
///path
    #[arg(short, long)]
    path:String,
    
    #[arg(short, long, default_value_t = 4)]
    count: u8,
}

fn main (){
    let args = Args::parse();
    
    let direction=match args.dir.parse::<char>().unwrap(){
        'N'=>Direction::North,
        'E'=>Direction::East,
        'S'=>Direction::South,
        'W'=>Direction::West,
        _=>{
            println!("Invalid direction, assumed north");
            Direction::North
        }
    };
    let robot=Robot::new(args.xpos.parse::<i32>().unwrap(), args.ypos.parse::<i32>().unwrap(), direction);
    let robot_final=robot.instructions(args.path.as_str());
    println!("{} {} {}",robot_final.position().0, robot_final.position().1, match robot_final.direction(){
        Direction::North=>'N',
        Direction::East=>'E',
        Direction::South=>'S',
        Direction::West=>'W'
    });
}
