pub fn parse_args<'a>(args: impl Iterator<Item = &'a String>) -> (bool, String) {
    args.fold((false, String::new()), |(edit, query), arg| {
        match arg.as_str() {
            "-e" | "--edit" => (true, query),
            _ => (edit, arg.clone()),
        }
    })
}
