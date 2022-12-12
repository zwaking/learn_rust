use csv::ReaderBuilder;
use std::usize;

use crate::line_limit::L99Interface;

pub fn read_csv(path: &str, week: usize) -> Vec<L99Interface> {
    let mut l99_vec: Vec<L99Interface> = Vec::new();

    // let path :&str = "C:/Users/waking/Downloads/w49.csv";
    let rdr = ReaderBuilder::new().has_headers(true).from_path(path);

    for result in rdr.unwrap().records() {
        let item = result.unwrap();

        let mut path = item.get(1).unwrap().to_string();
        if path.ends_with("/") {
            let rindex = path.rfind("/").unwrap();
            let _path = path.split_at(rindex);
            path = _path.0.to_string();
        }

        l99_vec.push(L99Interface {
            id: Default::default(),
            path: path,
            l99_time: item.get(7).unwrap().to_string().parse().unwrap(),
            week,
            application: item.get(0).unwrap().to_string(),
        });
    }

    return l99_vec;
}
