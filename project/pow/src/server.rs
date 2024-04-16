use tokio::sync::mpsc;

mod pb;
use pb::PowBuilder;

pub struct PowService {
    tx: mpsc::Sender<pb::Block>,
}

impl PowBuilder for PowService {
    
}

fn main() {
    println!("Hello, world!");
}
