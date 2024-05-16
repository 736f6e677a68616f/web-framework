mod database;

use database::DB;
use serde::Serialize;
#[derive(Debug, Serialize)]
struct Row {
    iid: u8
}
fn main() {
    let mut db = DB::init();
    let result: Vec<Row> = db.exec_sql("hxddz_pay2", "select iid from orders_20240619").unwrap();

}
