use crate::MS;

#[allow(dead_code)]
#[derive(Debug)]
struct MyRow {
    iid: u64
}

pub fn m() {
    let mut ms = MS.get().unwrap().lock().unwrap();
    let f = |iid| {
        MyRow {
            iid
        }
    };
    let rows = ms.db.exec_sql("hxddz_pay2", "select iid from orders_20230619",f).unwrap();
    println!("{:?}", rows);
}
