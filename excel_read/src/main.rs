use calamine::{Reader, open_workbook, Xlsx, Rows, DataType};
use std::{collections::HashMap};

fn filter_line95(rows: Rows<'_, DataType>) -> HashMap<String, f32> {
    let mut map_line95:HashMap<String, f32> = HashMap::new();

    let mut _rows = rows.filter(|row| {
        let _line95 = row[9].get_float().unwrap() as f32;
        return _line95 > 1.5;
    });

    return map_line95;
}

fn print_l95() {
    let path = "C:/Users/Administrator/Downloads/leopard.xlsx";
    let mut _workbook: Xlsx<_> = open_workbook(path).expect("cannot open file!");

    if let Some(Ok(range)) = _workbook.worksheet_range("msku维度数据"){
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!("Found {} cells in 'msku维度数据', including {} non empty cells", total_cells, non_empty_cells);
    }

    // let sheet_name = "statistics";
    // let sheet = _workbook.worksheet_range(&sheet_name).expect(&format!( "未找到目标sheet: {}",sheet_name));
    let sheet = _workbook.worksheet_range_at(0).expect("未找到可用的sheet");
    let mut count_row: usize = 0;
    let mut url_map:HashMap<String, f64> = HashMap::new();
    // 遍历sheet得到url + L95的map
    for row in sheet.unwrap().rows() {
        count_row += 1;
        // if count_row == 1 {
        //     continue;
        // }
        // print!("本行数据：");
        // println!("row={:?}, row[0]={:?}", row, row[0]);
        // 大于1.5s的才记录
        let value: f64 = row[9].get_float().unwrap_or(0.0);
        if value <= 1.5 {
            continue;
        }
        let mut key = row[0].to_string();
        if key.ends_with("/") {
            let rindex = key.rfind("/").unwrap();
            // println!("第{}行,最后一个\"/\"出现的位置{}", count_row, rindex);
            let _key = key.split_at(rindex);
            key = _key.0.to_string();
            // println!("截取后的url:{}",key);
        }
        if url_map.contains_key(&key){
            if url_map.get(&key).unwrap().lt(&value) {
                url_map.insert(key, value);
            }
        }else{
            url_map.insert(key, value);
        }
    }
    println!("一共{}行数据", count_row);
    let mut count_greater:usize = 0;
    // 输出所有url
    let mut _keys = Vec::from_iter(url_map.keys());
    // _keys.sort();
    _keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    for key in _keys {
        println!("url: {},L95: {}", key, url_map.get(key).unwrap());
        count_greater += 1;
    };
    // 输出L95大于1.5s的url以及L95时间
    // for entry in url_map {
    //     if entry.1 > 1.5 {
    //         count_greater += 1;
    //         println!("url: {},L95: {}", entry.0, entry.1);
    //     }
    // }
    println!("大于1.5s的有{}行",count_greater);
}

fn main() {
    print_l95();
}