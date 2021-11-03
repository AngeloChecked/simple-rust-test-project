mod some;

fn main() {
    println!("{:?}", some::sum(1,5));
}

#[cfg(test)]
mod tests {
    use super::some;

    #[test]
    fn sum() {
        assert_eq!(some::sum(1,5), 6);
    }
}
