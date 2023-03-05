pub fn to_url<T: ToString>(path: T) -> String {
    let mut path = path.to_string();
    
    if !path.contains("://") {
        path = format!("path:///{}", path);
    }
    
    path
        .replace("\\\\", "/")
        .replace("\\", "/")
}