mod helper_sql;
mod line_limit;

use calamine::{open_workbook, Reader, Xlsx};
use helper_sql::{build_insert_sql, build_update_sql};
use line_limit::{filter_timelimit, L99Interface};
use rusqlite::{params, Connection, Result};

fn analyze_limit() {
    let path = "C:/Users/waking/Downloads/fortunecat-w48.xlsx";
    let week: usize = 48;
    let application = String::from("fortuncat");
    let limit_second = 2.0;
    let limit_col_num: usize = 6;
    // 打开文件
    let mut _workbook: Xlsx<_> = open_workbook(path).expect("cannot open file!");
    let sheet = _workbook
        .worksheet_range_at(0)
        .expect("未找到可用的sheet")
        .unwrap();
    let vec_timelimit =
        filter_timelimit(sheet.rows(), limit_col_num, limit_second, week, application.as_str());
    let timelimit_count = vec_timelimit.len();
    let mut update_count: usize = 0;

    // 连接sqlite并入库
    let conn = Connection::open("slow_interface.sqlite").unwrap();
    for l99_interface in vec_timelimit {
        print!(
            "url: {}, L99: {}, application: {}",
            l99_interface.path, l99_interface.l99_time, l99_interface.application
        );
        // 查询是否存在
        let mut stmt = conn
            .prepare("select * from interface where path = ? and week = ?")
            .unwrap();
        let rs = stmt.query_row(params![l99_interface.path, week], |row| {
            // println!("数据信息{}", (row.get(1) as Result<String>).unwrap());
            return Ok(L99Interface {
                id: row.get(0).unwrap(),
                path: row.get(1).unwrap(),
                l99_time: row.get(2).unwrap(),
                week: row.get(3).unwrap(),
                application: row.get(4).unwrap(),
            }) as Result<L99Interface>;
        });

        let exists;
        if rs.is_ok() {
            exists = rs.unwrap();
        } else {
            exists = L99Interface::default();
        }
        let sql_str: String;
        if exists.id > 0 {
            println!(", 已存在: w{}", week);
            // 如果同一个week中接口已经存在,则比较L99时间大小,新的大就更新
            if exists.l99_time >= l99_interface.l99_time {
                continue;
            } else {
                sql_str = build_update_sql(exists.id, &l99_interface.l99_time, application.as_str());
            }
        } else {
            // 拼接插入sql
            sql_str = build_insert_sql(&l99_interface.path, &l99_interface.l99_time, week, application.as_str());
        }

        // println!("{}", sql_str);
        // update_count = conn.execute(&sql_str, ()).unwrap() + 1;
        match conn.execute(&sql_str, ()) {
            Ok(_o) => {
                update_count += 1;
                print!(", 插入成功: w{}", week);
            }
            Err(e) => {
                println!(", 插入失败, {}", e);
                continue;
            }
        };
        println!();
    }
    println!(
        "共{}个接口,超过阈值({}s)的接口{}个,插入{}条数据",
        sheet.rows().len(),
        limit_second,
        timelimit_count,
        update_count
    );
}

fn main() {
    analyze_limit();
}
