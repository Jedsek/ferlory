mod generated;

pub fn get(path: &'static str) -> &'static str {
    generated::POSTS.get(path).unwrap().to_owned()
}
