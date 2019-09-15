struct Task {
    id: String,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn it_works() {
        let uuid = Uuid::new_v4();
        let name = "Foo";
        let task = Task {
            id: uuid.to_string(),
            name: name.to_string(),
        };
        assert_eq!(uuid.to_string(), task.id);
        assert_eq!(name.to_string(), task.name);
    }
}
