use box_recursion_v2::*;

fn main() {
    let mut list = WorkEnvironment::new();
    list.add_worker(String::from("CEO"), String::from("Marie"));
    list.add_worker(String::from("Manager"), String::from("Monica"));
    list.add_worker(String::from("Normal Worker"), String::from("Ana"));
    list.add_worker(String::from("Normal Worker"), String::from("Alice"));
    println!("{:#?}", list);

    println!("{:?}", list.last_worker());

    list.remove_worker();
    list.remove_worker();
    list.remove_worker();
    println!("{:?}", list);
    list.remove_worker();
    println!("{:?}", list);
}

/*
WorkEnvironment {
    grade: Some(
        Worker {
            role: "Normal Worker",
            name: "Alice",
            next: Some(
                Worker {
                    role: "Normal Worker",
                    name: "Ana",
                    next: Some(
                        Worker {
                            role: "Manager",
                            name: "Monica",
                            next: Some(
                                Worker {
                                    role: "CEO",
                                    name: "Marie",
                                    next: None,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    ),
}
Some{{"Alice", "Normal Worker"}}
WorkEnvironment { grade: Some(Worker { role: "CEO", name: "Marie", next: None }) }
WorkEnvironment { grade: None }

*/