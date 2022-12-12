mod helper_db;
mod line_limit;
mod helper_csv;
mod helper_xlsx;

use helper_csv::read_csv;
use helper_xlsx::read_xlsx;
use line_limit::{filter_timelimit, L99Interface};
use log::{info, LevelFilter};
use simplelog::*;

use crate::helper_db::{check_exist, update};

fn analyze_limit(path: &str, limit_second:f64) {
    // 拆解文件名
    let path_split: Vec<&str> = path.split(&['/', '.', '-', 'w']).collect();
    // 从文件名中获取week
    let mut week: usize = 0;
    if week == 0 {
        week = path_split
            .get(path_split.len() - 2)
            .unwrap()
            .parse::<usize>()
            .expect("提取week失败");
    }
    // 从文件名中获取文件类型
    let suffix = path_split.get(path_split.len() - 1).unwrap().to_string();
    let interfece_vec;
    if suffix == String::from("xlsx") {
        interfece_vec = read_xlsx(path, week);
    }else{
        interfece_vec = read_csv(path, week);
    }
    let interfece_count = interfece_vec.len();
    let vec_timelimit = filter_timelimit(limit_second, interfece_vec);
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

    }
    info!(
        "共{}个接口,超过阈值({}s)的接口{}个,插入{}条数据",
        interfece_count,
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
    let path = "C:/Users/waking/Downloads/w50.csv";
    analyze_limit(path, 2.0);

    // let path = "C:/Users/waking/Downloads/w50.csv";
    // let l99_vec: Vec<L99Interface> = read_csv(path, 50);
    // println!("{}",l99_vec.len());
    // println!("{:?}",l99_vec);
    // let path = "C:/Users/waking/Downloads/w50.xlsx";
    // let l99_vec: Vec<L99Interface> = read_xlsx(path, 50);
    // println!("{}",l99_vec.len());
    // println!("{:?}",l99_vec);
}
