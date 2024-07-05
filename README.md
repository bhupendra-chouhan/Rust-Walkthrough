# **Rust Walkthrough - Part 1 and Part 2:**
Scroll down to see the output of both Part-1 and Part-2  (**Pre-work Project : "The Bouncy-Ball Game", Output Video** 👇)
![Pre-Work_Project_By_Bhupendra_Chouhan](https://github.com/Bhupendrachouhan19/Soroban_Internship_Bootcamp_Project/assets/78025043/36d7907f-bcb0-43b8-8dc4-67f3915a91ae)

---

## Project Folder Structure

```
├── src
│   ├── main_from_part_1.rs
│  └── main.rs    
├── target    
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```
---

## *Rust Walkthrough : Part - 1*

```
mkdir code_in_rust
cd code_in_rust
cargo new code_in_rust
```
Go to terminal and run ```cargo build``` -> it will add target, cargo.lock, and .gitignore

---
### Rust Basics, learned from Part-1 video:


Make sure to rename the file ```main_from_part_1.rs``` to ```main.rs``` before compiling and running it.

**Source File Location: /src/main_from_part_1.rs** 
```

// Everything inside in our smartcontract happens inside this 'main' function.
fn main() {
    // variables types in RUST:
    // let -> immutable
    // let mut -> mutable

    // basic data types ->  Rust is a statically typed language.

    let x: i32 = 16;
    /*
        let => Immutable Variable type
        x => Variable name
        : => colon for explanation
        i32 => Integer of 32 bits
        = => Assignment operator
        16 => Value stored in the varaible 'x'
    */

    // In order to print the output out, we need to write a macro =>  'println!' macro
    println!("{}", x); // this is the first version of how we use println!

    let y: String = String::from("Hello, Soroban!"); // mutable string
    let z: &str = "Hello, Stellar!"; // immutable string

    println!("{y}"); // this is the second version of the macro
    println!("{z}"); // this is the second version of the macro


    /*
        => Function in Rust:
        1) fn -> private function
        2) pub fn -> public function
    */

    // Function Declaration:
    pub fn event() {
        // public function:

        let name: String = String::from("James");
        println!("{}", name);
    }

    event(); // Function call


    /* Implementing Stucts: */
    let e: EventForKids = EventForKids {
        name: String::from("KidsCo"),
        date: String::from("15-5-2024"),
        number_of_participants: 1000,
        place: String::from("NY, USA")
    };

    
    /* Implementing Stucts: */
    // Add your enum in here. . . .
}


/*
    => Stucts in Rust: Compiling many items in one class.
*/
// struct declaration:
struct EventForKids {
    name: String,
    date: String,
    number_of_participants: u32, // unsigned 32 bit
    place: String,
}


/*
    enums in Rust: Compiling errors in one class.
    -> In enums we don't actually put types, but in stucts we do need to put types.
*/
// enum declaration:
enum ErrorsForEvent {
    NoEvent,
    CancelledEvent,
    EventType
}
```

**Output:**
```
16
Hello, Soroban!
Hello, Stellar!
James
``` 


---

## *Rust Walkthrough : Part - 2*
Creating 'The Bouncy-Ball Game' in Rust.

**Source File Location: /src/main_from_part_1.rs** 

```
use std::fmt;
use std::fmt::{Display, Formatter, Error};

enum VertDir {
    Up,
    Down
}

enum HorizDir {
    Left,
    Right
}


struct Ball {
    x: i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: i32,
    height: i32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame : Frame {
                width : 63,
                height: 31
            },

            ball : Ball {
                x : 44,
                y : 21,
                vert_dir : VertDir::Down,
                horiz_dir : HorizDir::Right
            }
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        }
        else if frame.width <= self.x {
            self.horiz_dir = HorizDir::Left;
        }
        else if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        }
        else if frame.height <= self.y {
            self.vert_dir = VertDir::Up;
        }
        else {
            return;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "x");
        for _ in 0..64 { write!(fmt, "-"); }
        for y in 0..32 {
            for x in 0..64 {
                if self.ball.x == x as i32 && self.ball.y == y as i32 {
                    write!(fmt, "0");
                }
                if x == 0 {write!(fmt, "|");} 
                else if x != 0 && y != 31 {write!(fmt, " ");}
                else { write!(fmt, "-"); }
            }

            write!(fmt, "\n");
        }
        write!(fmt, "\n")
    }
}

fn main() {
    let mut new_game = Game::new();
    let sleep_time = std::time::Duration::from_millis(33);
    loop {
        println!("{}", new_game);
        new_game.step();
        std::thread::sleep(sleep_time);
        println!("{} {}", new_game.ball.x, new_game.ball.y); 
    }
}
```
**Output:**
***Pre-work Project Video output embedded at the top*** 👆
