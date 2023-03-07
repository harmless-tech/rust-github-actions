fn main() {
    println!("Hello, world!");
    let useless =
        "22";
}

#[cfg(test)]
mod test {
    #[test]
    fn add() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }
}
