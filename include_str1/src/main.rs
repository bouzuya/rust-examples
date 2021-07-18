fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(include_str!("../my_files/message.txt"), "Hello, world!\n");
    }
}
