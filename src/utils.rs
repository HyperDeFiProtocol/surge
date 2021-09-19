pub fn key(s: &str) -> &str {
    let vec: Vec<&str> = s.split("=").collect();

    vec[0].trim()
}
