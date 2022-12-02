mod helper_db;
mod line_limit;

use calamine::{open_workbook, Reader, Xlsx};
use line_limit::{filter_timelimit, L99Interface};
use log::{info, LevelFilter};
use simplelog::*;

use crate::helper_db::{check_exist, update};

fn analyze_limit() {
    let path = "C:/Users/waking/Downloads/adjustment-w47.xlsx";
    let path_split: Vec<&str> = path.split(&['/', '.', '-', 'w']).collect();
    // let path_split: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
    // assert_eq!(path_split, ["2020", "11", "03", "23", "59"]);
    // let path_split = split().src(path).delimeter(".").perform().map(|e| String::from(e)).collect::<Vec<String>>();
    // for ele in path_split.clone() {
    //     debug!("{}", ele);
    // }
    // let path_split = path_split.clone();
    let mut week: usize = 0;
    if week == 0 {
        week = path_split
            .get(path_split.len() - 2)
            .unwrap()
            .parse::<usize>()
            .expect("提取week失败");
    }
    let application = path_split.get(path_split.len() - 4).unwrap().to_string();
    // let name = path_split.get(path_split.len() - 1);
    // debug!("{}", name);
    // let application_and_week: Vec<&str> = name.split("-").collect();
    // debug!("{}+{}", _week, _application);
    // let week: usize = 48;
    // let application = String::from("fortuncat");
    let limit_second = 2.0;
    let limit_col_num: usize = 6;
    // 打开文件
    let mut _workbook: Xlsx<_> = open_workbook(path).expect("cannot open file!");
    let sheet = _workbook
        .worksheet_range_at(0)
        .expect("未找到可用的sheet")
        .unwrap();
    let vec_timelimit = filter_timelimit(
        sheet.rows(),
        limit_col_num,
        limit_second,
        week,
        application.as_str(),
    );
    let timelimit_count = vec_timelimit.len();
    let mut update_count: usize = 0;

    let mut log_info: String;
    for mut l99_interface in vec_timelimit {
        log_info = format!(
            "url: {}, L99: {}, application: {}, week: w{}",
            l99_interface.path, l99_interface.l99_time, l99_interface.application, l99_interface.week
        );

        // 检查是否存在
        let l99_exist = check_exist(l99_interface.clone());

        if l99_exist.id > 0 {
            log_info += ", 已存在";
            info!("{}", log_info);

            // 如果同一个week中接口已经存在,则比较L99时间大小,新的大就更新
            if l99_exist.l99_time >= l99_interface.l99_time {
                continue;
            }else{
                let _l99_interface = L99Interface{
                    id: l99_exist.id,
                    ..l99_interface
                };
                l99_interface = _l99_interface;
            }
        };

        if update(l99_interface) {
            if l99_exist.id > 0 {
                log_info += ", 已更新";
            }else{
                log_info += ", 已新增";
            }
            update_count += 1;
        }

        info!("{}", log_info);

        // log_str = format!(
        //     "url: {}, L99: {}, application: {}",
        //     l99_interface.path, l99_interface.l99_time, l99_interface.application
        // );
        // // 查询是否存在
        // let mut stmt = conn
        //     .prepare("select * from interface where path = ? and week = ?")
        //     .unwrap();
        // let rs = stmt.query_row(params![l99_interface.path, week], |row| {
        //     // println!("数据信息{}", (row.get(1) as Result<String>).unwrap());
        //     return Ok(L99Interface {
        //         id: row.get(0).unwrap(),
        //         path: row.get(1).unwrap(),
        //         l99_time: row.get(2).unwrap(),
        //         week: row.get(3).unwrap(),
        //         application: row.get(4).unwrap(),
        //     }) as Result<L99Interface>;
        // });

        // let exists;
        // if rs.is_ok() {
        //     exists = rs.unwrap();
        // } else {
        //     exists = L99Interface::default();
        // }
        // let sql_str: String;
        // if exists.id > 0 {
        //     log_str += &format!(", 已存在: w{}", week);
        //     debug!("{}", log_str);
        //     // 如果同一个week中接口已经存在,则比较L99时间大小,新的大就更新
        //     if exists.l99_time >= l99_interface.l99_time {
        //         continue;
        //     } else {
        //         sql_str =
        //             build_update_sql(exists.id, &l99_interface.l99_time, application.as_str());
        //     }
        // } else {
        //     // 拼接插入sql
        //     sql_str = build_insert_sql(
        //         &l99_interface.path,
        //         &l99_interface.l99_time,
        //         week,
        //         application.as_str(),
        //     );
        // }

        // // println!("{}", sql_str);
        // // update_count = conn.execute(&sql_str, ()).unwrap() + 1;
        // match conn.execute(&sql_str, ()) {
        //     Ok(_o) => {
        //         update_count += 1;
        //         log_str += &format!(", 插入成功: w{}", week);
        //         debug!("{}", log_str);
        //     }
        //     Err(e) => {
        //         warn!(", 插入失败, {}", e);
        //         continue;
        //     }
        // };
    }
    info!(
        "共{}个接口,超过阈值({}s)的接口{}个,插入{}条数据",
        sheet.rows().len(),
        limit_second,
        timelimit_count,
        update_count
    );
}

fn main() {
    // 初始化日志配置
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    // 跑
    analyze_limit();
}
