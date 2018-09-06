mod doc {
    use Doc;
    use SpanLike;
    use SpanType;

    #[test]
    fn from_empty_string() {
        let doc = Doc::from_string(String::new());

        assert_eq!(doc, Doc::new());
    }

    #[test]
    fn action_from_roman_string() {
        let action_str = "Once upon a time in a land far, far away...";
        let doc = Doc::from_string(action_str.to_string());

        assert_eq!(doc.to_string(), action_str.to_string());
        assert_eq!(doc.elements().count(), 1);

        let mut positions = doc.positions();
        let pos = positions.next().unwrap();
        assert_eq!(pos.span_type(), SpanType::Doc);
        assert_eq!(pos.char(), 'O');
        assert_eq!(positions.next().unwrap().char(), 'n');
        assert_eq!(positions.next().unwrap().char(), 'c');
        assert_eq!(positions.next().unwrap().char(), 'e');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'u');
        assert_eq!(positions.next().unwrap().char(), 'p');
        assert_eq!(positions.next().unwrap().char(), 'o');
        assert_eq!(positions.next().unwrap().char(), 'n');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'a');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 't');
        assert_eq!(positions.next().unwrap().char(), 'i');
        assert_eq!(positions.next().unwrap().char(), 'm');
        assert_eq!(positions.next().unwrap().char(), 'e');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'i');
        assert_eq!(positions.next().unwrap().char(), 'n');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'a');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'l');
        assert_eq!(positions.next().unwrap().char(), 'a');
        assert_eq!(positions.next().unwrap().char(), 'n');
        assert_eq!(positions.next().unwrap().char(), 'd');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'f');
        assert_eq!(positions.next().unwrap().char(), 'a');
        assert_eq!(positions.next().unwrap().char(), 'r');
        assert_eq!(positions.next().unwrap().char(), ',');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'f');
        assert_eq!(positions.next().unwrap().char(), 'a');
        assert_eq!(positions.next().unwrap().char(), 'r');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), 'a');
        assert_eq!(positions.next().unwrap().char(), 'w');
        assert_eq!(positions.next().unwrap().char(), 'a');
        assert_eq!(positions.next().unwrap().char(), 'y');
        assert_eq!(positions.next().unwrap().char(), '.');
        assert_eq!(positions.next().unwrap().char(), '.');
        assert_eq!(positions.next().unwrap().char(), '.');

        assert_eq!(positions.next(), None);
    }

    #[test]
    fn action_from_interntl_string() {
        let action_str = "占 占占点";
        let doc = Doc::from_string(action_str.to_string());

        let mut positions = doc.positions();
        let pos = positions.next().unwrap();
        assert_eq!(pos.span_type(), SpanType::Doc);
        assert_eq!(pos.char(), '占');
        assert_eq!(positions.next().unwrap().char(), ' ');
        assert_eq!(positions.next().unwrap().char(), '占');
        assert_eq!(positions.next().unwrap().char(), '占');
        assert_eq!(positions.next().unwrap().char(), '点');

        assert_eq!(positions.next(), None);

        assert_eq!(doc.to_string(), action_str.to_string());
    }
}
