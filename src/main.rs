fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn add() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }
}
