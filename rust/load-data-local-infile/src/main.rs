use std::io::Write;
use mysql::*;
use mysql::prelude::*;

fn main() {
    let url = "mysql://user:password@127.0.0.1:3307/testdb".to_string();
    let opts = Opts::from_url(&*url).unwrap();
    let mut conn = Conn::new(opts).unwrap();
    conn.query_drop("CREATE TEMPORARY TABLE tbl (
        `id` INT,
        `text` TEXT
    )").unwrap();
    conn.set_local_infile_handler(Some(LocalInfileHandler::new(|_, stream| {
        stream.write_all(format!("{}\t{}\n", 1, "hoge").as_bytes())?;
        Ok(())
    })));

    match conn.query_drop("LOAD DATA LOCAL INFILE 'file_name' INTO TABLE tbl") {
        Ok(_) => {}
        Err(err) => panic!("ERROR {}", err),
    }

    let result = conn.query_iter("SELECT `id`,`text` FROM tbl").unwrap();
    for row in result {
        println!("{:?}", row.unwrap());
    }
}
