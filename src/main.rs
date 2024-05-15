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
    // Add your enum in here....
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
*/
// enum declaration:
enum ErrorsForEvent {
    NoEvent,
    CancelledEvent,
    EventType
}
