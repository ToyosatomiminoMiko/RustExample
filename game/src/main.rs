use std::fmt;
//use std::fmt::Error;
use std::io;
use std::num::ParseIntError;

fn stoi(s: &str) -> Result<u8, ParseIntError> {
    /*
    字符串转无符号8位整数
     */
    s.trim().parse::<u8>()
}

#[derive(Debug)]
pub struct Point2D {
    /*
    坐标
     */
    x: u8,
    y: u8,
}
impl Point2D {
    pub fn create() -> Self {
        let mut input: String = String::new();
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
            /*
            如果想使一个可恢复错误按不可恢复错误处理
            Result 类提供了两个办法:
            `unwrap()` 和 `expect(message: &str)`
            expect 能够向 panic! 宏发送一段指定的错误信息
            */
            let v: Vec<&str> = input.trim().split(',').collect();
            // 移除字符串前后空格后以逗号分隔解析输入坐标
            //println!("{:?},{:?}", stoi(v[0]), stoi(v[1]));
            match stoi(v[0]) {
                Ok(x0) => match stoi(v[1]) {
                    Ok(y0) => {
                        break Point2D { x: x0, y: y0 };
                    }
                    Err(_) => continue,
                },
                Err(_) => continue,
            };
        }
    }
}

/*
struct Game {
    players: [Player; 2],
}
*/


    



#[derive(Clone)]
struct Player {
    name: String,
    c: u8,
    //board: &'a mut Map,
    //board: &mut Map,
}
impl Player {
    /* // new!!
    fn goto(&mut self) {
        loop {
            let c = Point2D::create();

            if self.board.index[c.x as usize][c.y as usize] != 0 {
                println!("这里已经有人了.");
                continue;
            }
            self.board.index[c.x as usize][c.y as usize] = self.c;
            println!("player [{}] goto the [{},{}]", self.name, c.x, c.y);
            break;
        }
    }*/
    /*
    fn goto(&mut self) {
        loop {
            let c = Point2D::create();
            if self.board.index[c.x as usize][c.y as usize] != 0 {
                println!("这里已经有人了.");
                continue;
            }
            self.board.index[c.x as usize][c.y as usize] = self.c;
            println!("player [{}] goto the [{},{}]", self.name, c.x, c.y);
            break;
        }
    }*/
}

//#[derive(Clone)]
pub struct Map<'a>{
    index: [[u8; 10]; 10],
    //game: Game,
    players: [&'a  Player; 2],
}

// 重写`Display` 显示棋盘
impl fmt::Display for Map<'_>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        for n in self.index {
            for m in n {
                if m == 1 {
                    s += "@";
                } else if m == 2 {
                    s += "#";
                } else {
                    s += " ";
                }
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

impl Map<'_>{
    /* // old??
    fn goto(&mut self, p: Player, c: Point2D) -> Result<Self, bool> {
        //let c = Point2D::create();
        if self.index[c.x as usize][c.y as usize] != 0 {
            println!("这里已经有人了.");
            return Err(false);
        }
        self.index[c.x as usize][c.y as usize] = p.c;
        println!("player [{}] goto the [{},{}]", p.name, c.x, c.y);
        return Ok(*self);
    }*/
    fn goto(&mut self, p: Player) {
        loop {
            let c: Point2D = Point2D::create();
            if self.index[c.x as usize][c.y as usize] != 0 {
                println!("这里已经有人了.");
                continue;
            }
            self.index[c.x as usize][c.y as usize] = p.c;
            println!("player [{}] goto the [{},{}]", p.name, c.x, c.y);
            break;
        }
    }
}

fn main() {
    let p1: Player = Player {
        name: String::from("nmsl"),
        c: 1,
        //board: &mut b1,
    };
    let p2: Player = Player {
        name: String::from("cnmd"),
        c: 2,
        //board: &mut b1,
    };
    let mut b1: Map = Map {
        index: [[0; 10]; 10],
        players: [&p1 ,&p2],
    };
    loop{
        let mut i: usize = 0;
        b1.goto(*b1.players[i]);
    }

    /* 
    p1.goto();
    println!("{}", b1);
    p2.goto();
    println!("{}", b1);
    */
    /*
    let mut a:[[u8;10];10] = [[0; 10]; 10];
    // 美化打印
    let m:u8 = 1;
    let n:u8 = 1;
    let c:u8 = 2;
    a[m as usize][n as usize] = c;
    println!("{:?}", a[m as usize][n as usize]);*/
}
