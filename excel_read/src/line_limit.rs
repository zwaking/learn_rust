use core::fmt;

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

impl fmt::Debug for L99Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("L99Interface")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("l99_time", &self.l99_time)
            .field("week", &self.week)
            .field("application", &self.application)
            .finish()
    }
}

pub fn filter_timelimit(limit_second: f64, l99_vec: Vec<L99Interface>) -> Vec<L99Interface> {
    let mut vec_timelimit: Vec<L99Interface>;

    // 过滤超过阈值的数据
    vec_timelimit = l99_vec
        .iter()
        .filter(|row| {
            return row.l99_time > limit_second;
        })
        .map(|item| item.to_owned())
        .collect();

    // 根据path排序
    vec_timelimit.sort_by_key(|item| item.path.to_lowercase());

    return vec_timelimit;
}
