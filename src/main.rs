enum IpAddrKind{
    V4,
    V6
}
// Easier and better way
enum IpAddr{
    V4(String),
    V6(String),
}

// Multiple types in 1 enum possible 
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

// Implementation works same for structs and enums 
impl Message{
    fn call(&self){
        // method body
    }
}

let m :String = Message::Write(String::from("Hello"));
m.call();

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
fn main() {
    // Enum Values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Route function use 
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Option Enum

    let some_number = Some(5);
    let some_char = Some('s');

    let absent_number :Option<i32> = None;
}

// Function to use Enum IpAddrKind
fn route(ip_kind:IpAddrKind){

}
