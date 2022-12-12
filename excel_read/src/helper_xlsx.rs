use calamine::{open_workbook, Reader, Xlsx};

use crate::line_limit::L99Interface;

pub fn read_xlsx(path: &str, week: usize) -> Vec<L99Interface> {
    let mut l99_vec = Vec::<L99Interface>::new();

    // 打开文件
    let mut _workbook: Xlsx<_> = open_workbook(path).expect("cannot open file!");
    let sheet = _workbook
        .worksheet_range_at(0)
        .expect("未找到可用的sheet")
        .unwrap();

    // 循环数据组转成新的vector
    let mut i = 1;
    for row in sheet.rows() {
        if i == 1{
            i = 0;
            continue;
        }


        let mut path = row[1].to_string();
        if path.ends_with("/") {
            let rindex = path.rfind("/").unwrap();
            // println!("第{}行,最后一个\"/\"出现的位置{}", count_row, rindex);
            let _path = path.split_at(rindex);
            path = _path.0.to_string();
            // println!("截取后的url:{}",key);
        }
        let value: f64 = row[7].get_float().unwrap_or(0.0);
        let application: String = row[0].to_string();
        l99_vec.push(L99Interface {
            id: 0,
            path: path,
            l99_time: value,
            week: week,
            application: application,
        });
    }

    return l99_vec;
}
