mod generated;

pub fn get(path: &str) -> &'static str {
    generated::POSTS.get(&path).unwrap().to_owned()
}
