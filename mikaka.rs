use std::io::{self,Read};
mod libmikaka;

fn main(){
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    for c in buf.chars() {
        print!("{}",libmikaka::to_mikaka(c.to_string().as_str()));
    }
}

