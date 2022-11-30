use calamine::{DataType, Rows};

pub struct L99Interface {
    pub id: usize,
    pub path: String,
    pub l99_time: f64,
    pub week: usize,
    pub application: String,
}

impl Default for L99Interface {
    fn default() -> Self {
        Self {
            id: Default::default(),
            path: Default::default(),
            l99_time: Default::default(),
            week: Default::default(),
            application: Default::default(),
        }
    }
}

// impl L99Interface {
//     fn default() -> Self {
//         Default::default()
//     }
// }

pub fn filter_timelimit(
    rows: Rows<'_, DataType>,
    limit_col_num: usize,
    limit_second: f64,
    week: usize,
    application: &str,
) -> Vec<L99Interface> {
    let mut vec_timelimit = Vec::<L99Interface>::new();

    // 过滤超过阈值的数据
    let _rows = rows.filter(|row| {
        let _timelimit = row[limit_col_num].get_float().unwrap_or(0.0) as f64;
        return _timelimit > limit_second;
    });

    // 循环超过阈值的数据组转成新的vector
    for row in _rows {
        let mut path = row[0].to_string();
        if path.ends_with("/") {
            let rindex = path.rfind("/").unwrap();
            // println!("第{}行,最后一个\"/\"出现的位置{}", count_row, rindex);
            let _path = path.split_at(rindex);
            path = _path.0.to_string();
            // println!("截取后的url:{}",key);
        }
        let value: f64 = row[limit_col_num].get_float().unwrap_or(0.0);
        let _limit = L99Interface {
            id: 0,
            path: path,
            l99_time: value,
            week: week,
            application: application.to_string(),
        };
        vec_timelimit.push(_limit);
    }

    // 根据path排序
    vec_timelimit.sort_by_key(|item| item.path.to_lowercase());

    return vec_timelimit;
}

// fn print_l95() {
//     let path = "C:/Users/waking/Downloads/statistics-w48.xlsx";
//     let week: usize = 48;
//     let limit_second = 2.0;
//     let limit_col_num: usize = 6;
//     let mut _workbook: Xlsx<_> = open_workbook(path).expect("cannot open file!");

//     // if let Some(Ok(range)) = _workbook.worksheet_range("msku维度数据") {
//     //     let total_cells = range.get_size().0 * range.get_size().1;
//     //     let non_empty_cells: usize = range.used_cells().count();
//     //     println!(
//     //         "Found {} cells in 'msku维度数据', including {} non empty cells",
//     //         total_cells, non_empty_cells
//     //     );
//     // }

//     // let sheet_name = "statistics";
//     // let sheet = _workbook.worksheet_range(&sheet_name).expect(&format!( "未找到目标sheet: {}",sheet_name));
//     let sheet = _workbook.worksheet_range_at(0).expect("未找到可用的sheet");
//     let mut count_row: usize = 0;
//     let mut url_map: HashMap<String, f64> = HashMap::new();
//     // 遍历sheet得到url + L95的map
//     for row in sheet.unwrap().rows() {

//         count_row += 1;
//         // if count_row == 1 {
//         //     continue;
//         // }
//         // print!("本行数据：");
//         // println!("row={:?}, row[0]={:?}", row, row[0]);
//         // 大于limit_seconds的才记录
//         let value: f64 = row[limit_col_num].get_float().unwrap_or(0.0);
//         if value <= limit_second {
//             continue;
//         }

//         let mut key = row[0].to_string();
//         if key.ends_with("/") {
//             let rindex = key.rfind("/").unwrap();
//             // println!("第{}行,最后一个\"/\"出现的位置{}", count_row, rindex);
//             let _key = key.split_at(rindex);
//             key = _key.0.to_string();
//             // println!("截取后的url:{}",key);
//         }

//         if url_map.contains_key(&key) {
//             if url_map.get(&key).unwrap().lt(&value) {
//                 url_map.insert(key, value);
//             }
//         } else {
//             url_map.insert(key, value);
//         }
//     }
//     println!("一共{}行数据", count_row);
//     let mut count_greater: usize = 0;
//     // 输出所有url
//     let mut _keys = Vec::from_iter(url_map.keys());
//     // _keys.sort();
//     _keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
//     let mut count: usize = 0;
//     for key in _keys {
//         print!("url: {}, L99: {}", key, url_map.get(key).unwrap());
//         count_greater += 1;

//         // 连接sqlite并入库
//         let conn = Connection::open("slow_interface.sqlite").unwrap();

//         // 查询是否存在
//         let mut stmt = conn
//             .prepare("select * from interface where path = ? and week = ?")
//             .unwrap();
//         let rs = stmt.query_row(params![key, week], |row| {
//             // println!("数据信息{}", (row.get(1) as Result<String>).unwrap());
//             return Ok(L99Interface {
//                 id: row.get(0).unwrap(),
//                 path: row.get(1).unwrap(),
//                 l99_time: row.get(2).unwrap(),
//                 week: row.get(3).unwrap(),
//             }) as Result<L99Interface>;
//             // return row.get(0) as Result<l99_interface>;
//         });

//         let exists = rs.unwrap();
//         let sql_str: String;
//         if exists.id > 0 {
//             println!(" ,已存在: w{}", week);
//             // 如果同一个week中接口已经存在,则比较L99时间大小,新的大就更新
//             if exists.l99_time >= *url_map.get(key).unwrap() {
//                 continue;
//             } else {
//                 sql_str = build_update_sql(exists.id, url_map.get(key).unwrap());
//             }
//         } else {
//             // 拼接插入sql
//             sql_str = build_insert_sql(key, url_map.get(key).unwrap(), week);
//         }

//         println!("{}", sql_str);
//         count = conn.execute(&sql_str, ()).unwrap() + 1;
//         match conn.execute(&sql_str, ()) {
//             Ok(_o) => {
//                 count += 1;
//                 println!("插入成功: w{}", week);
//             }
//             Err(e) => {
//                 println!("插入失败, {}", e);
//                 continue;
//             }
//         };
//     }
//     println!(
//         "共{}个接口,超过阈值({}s)的接口{}个,插入{}条数据",
//         count_row, limit_second, count_greater, count
//     );
//     // 输出超过阈值的数据
//     for entry in url_map {
//         println!("url: {},L95: {}", entry.0, entry.1);
//     }
// }
