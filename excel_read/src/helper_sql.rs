pub fn build_insert_sql(path: &str, l99_time: &f64, week: usize, application: &str) -> String {
    let sql_insert = format!(
        "insert into interface('path','l99_time','week','application') values('{}',{},{},'{}')",
        path, l99_time, week, application
    );
    return sql_insert;
}

pub fn build_update_sql(id: usize, l99_time: &f64, application: &str) -> String {
    let sql_update = format!(
        "update interface set l99_time = {} where id = {} and application = '{}'",
        l99_time, id, application
    );
    return sql_update;
}
