use calamine::{DataType, Rows};

pub struct L99Interface {
    pub id: usize,
    pub path: String,
    pub l99_time: f64,
    pub week: usize,
    pub application: String,
}

impl Default for L99Interface {
    fn default() -> L99Interface {
        L99Interface {
            id: Default::default(),
            path: Default::default(),
            l99_time: Default::default(),
            week: Default::default(),
            application: Default::default(),
        }
    }
}

impl Clone for L99Interface {
    fn clone(&self) -> L99Interface {
        L99Interface {
            id: self.id,
            path: self.path.to_string(),
            l99_time: self.l99_time,
            week: self.week,
            application: self.application.to_string(),
        }
    }
}

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
