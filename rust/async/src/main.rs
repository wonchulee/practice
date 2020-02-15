use futures::executor::block_on;
use futures::join;
use std::io;

mod asyncop;

use asyncop::{Task1, Task2};

fn main() {
    block_on(asyncop());
}

async fn asyncop() {
    join!(fut1(), fut2());
}

// Future 1
fn fut1() -> Task1 {
    Task1 { count : 0 }
}

// Future 2
fn fut2() -> Task2 {
    Task2 { count : 0 }
}
