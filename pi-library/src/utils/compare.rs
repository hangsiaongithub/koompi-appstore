#[allow(clippy::comparison_chain)]
pub fn compare(local: u32, online: u32) -> i8 {
    if local == online {
        0
    } else if local < online {
        1
    } else {
        -1
    }
}
