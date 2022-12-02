use crate::line_limit::L99Interface;
use log::{debug, error};

use rusqlite::{params, Connection, Result};

fn get_connection() -> Connection {
    let conn = Connection::open("slow_interface.sqlite").unwrap();
    return conn;
}

pub fn check_exist(l99_interface: L99Interface) -> L99Interface {
    let _interface: L99Interface;

    let query_sql = "select * from interface where path = ? and week = ?";
    let rs = get_connection().prepare(query_sql).unwrap().query_row(
        params![l99_interface.path, l99_interface.week],
        |row| {
            return Ok(L99Interface {
                id: row.get(0).unwrap(),
                path: row.get(1).unwrap(),
                l99_time: row.get(2).unwrap(),
                week: row.get(3).unwrap(),
                application: row.get(4).unwrap(),
            }) as Result<L99Interface>;
        },
    ) as Result<L99Interface>;

    if rs.is_ok() {
        _interface = rs.unwrap();
    } else {
        _interface = L99Interface::default();
    }

    return _interface;
}

pub fn update(l99_interface: L99Interface) -> bool {
    let mut success = false;

    let mut log_str;
    let week = l99_interface.week;
    if l99_interface.id > 0 {
        log_str = build_update_sql(
            l99_interface.id,
            &l99_interface.l99_time,
            &l99_interface.application,
        );
    } else {
        log_str = build_insert_sql(
            &l99_interface.path,
            &l99_interface.l99_time,
            week,
            &l99_interface.application,
        );
    }
    match get_connection().execute(&log_str, ()) {
        Ok(_o) => {
            log_str += &format!(", 执行成功: w{}", week);
            debug!("{}", log_str);
            success = true;
        }
        Err(e) => {
            log_str += &format!(", 执行失败: w{}", week);
            error!("{}{}", log_str, e);
        }
    };

    return success;
}

fn build_insert_sql(path: &str, l99_time: &f64, week: usize, application: &str) -> String {
    let sql_insert = format!(
        "insert into interface('path','l99_time','week','application') values('{}',{},{},'{}')",
        path, l99_time, week, application
    );
    return sql_insert;
}

fn build_update_sql(id: usize, l99_time: &f64, application: &str) -> String {
    let sql_update = format!(
        "update interface set l99_time = {} where id = {} and application = '{}'",
        l99_time, id, application
    );
    return sql_update;
}
