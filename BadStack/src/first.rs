mod test {
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(). None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(),Some(3));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}