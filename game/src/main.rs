use std::fmt;
//use std::fmt::Error;
use std::io;
//use std::num::ParseIntError;

fn string_to_static_str(s: String) -> &'static str {
    //将`String`转换为`&str`
    Box::leak(s.into_boxed_str())
}

fn stoi(s: &str) -> u8 {
    //字符串转无符号8位整数
    s.trim().parse::<u8>().expect("请输入坐标!")
}
/*
fn stoi(s: &str) -> Result<u8, ParseIntError> {
    //字符串转无符号8位整数
    s.trim().parse::<u8>()
}*/

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

            return Point2D {
                x: stoi(v[0]),
                y: stoi(v[1]),
            };
            /*
            match stoi(v[0]) {
                Ok(x0) => match stoi(v[1]) {
                    Ok(y0) => {
                        break Point2D { x: x0, y: y0 };
                    }
                    Err(_) => {
                        println!("请输入坐标!");
                        continue;
                    }
                },
                Err(_) => {
                    println!("请输入坐标!");
                    continue;
                }
            };*/
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
pub struct Map<'a> {
    index: [[u8; 10]; 10],
    players: [&'a Player; 2],
}

// 重写`Display` 显示棋盘
impl fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        println!("  0 1 2 3 4 5 6 7 8 9");
        let mut r: u8 = 0;
        for n in self.index {
            s += string_to_static_str(r.to_string());
            for m in n {
                if m == 1 {
                    s += " @";
                } else if m == 2 {
                    s += " #";
                } else {
                    s += " +";
                }
            }
            s += "\n";
            r += 1;
        }
        write!(f, "{}", s)
    }
}

impl Map<'_> {
    fn goto(&mut self, p: Player) {
        loop {
            let c: Point2D = Point2D::create();
            if (c.x > 9) | (c.y > 9) {
                println!("超出范围,请重新输入.");
                continue;
            } else if self.index[c.x as usize][c.y as usize] != 0 {
                println!("这里已经有人了.");
                continue;
            } else {
                self.index[c.x as usize][c.y as usize] = p.c;
                println!("玩家[{}]进[{},{}].", p.name, c.x, c.y);
                break;
            }
        }
    }
}

fn main() {
    let p1: Player = Player {
        name: String::from("nmsl"),
        c: 1,
    };
    let p2: Player = Player {
        name: String::from("cnmd"),
        c: 2,
    };
    let mut b1: Map = Map {
        index: [[0; 10]; 10],
        players: [&p1, &p2],
    };
    loop {
        b1.goto(b1.players[0].clone());
        println!("{:#}", b1);
        b1.goto(b1.players[1].clone());
        println!("{:#}", b1);
    }
}
