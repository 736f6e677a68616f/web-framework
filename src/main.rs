mod database;
mod tt;

use std::sync::{Mutex, OnceLock};
use database::DB;
static MS: OnceLock<Mutex<Ms>> = OnceLock::new();

#[derive(Debug)]
struct Ms {
    db: DB
}
impl Ms {
    fn new() -> Self {
        Ms {
            db: DB::init()
        }
    }
}

fn init_ms() {
    MS.get_or_init(|| Mutex::new(Ms::new()));
}

#[tokio::main]
async fn main() {
    init_ms();
    tt::m();
}
