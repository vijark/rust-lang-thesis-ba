trait Human {
    fn first_name(&self) -> &String;
    fn family_name(&self) -> &String;
    fn full_name(&self) -> String {
        format!("{} {}", &self.first_name(), &self.family_name())
    }
}

struct Employee {
    first_name: String,
    family_name: String,
}

impl Employee {
    fn new(first_name: &str, family_name: &str) -> Employee {
        Employee {
            first_name: String::from(first_name),
            family_name: String::from(family_name),
        }
    }
}

impl Human for Employee {
    fn first_name(&self) -> &String {
        &self.first_name
    }
    fn family_name(&self) -> &String {
        &self.family_name
    }
}

struct Manager {
    emp: Employee,
    group: Vec<Box<dyn Human>>,
    level: i8,
}

impl Manager {
    fn new(first_name: &str, family_name: &str, group: Vec<Box<dyn Human>>, level: i8) -> Manager {
        Manager {
            emp: Employee::new(first_name, family_name),
            group: group,
            level: level,
        }
    }
}

impl Human for Manager {
    fn first_name(&self) -> &String {
        &self.emp.first_name
    }
    fn family_name(&self) -> &String {
        &self.emp.family_name
    }
    fn full_name(&self) -> String {
        format!("{} {} (level {})", &self.emp.first_name, &self.emp.family_name, &self.level)
    }
}

fn foo(c: &dyn Human) {
    println!("foo: {}, {}", c.family_name(), c.first_name());
}

fn foo2(c1: &dyn Human, c2: &dyn Human) {
    print!("foo2: ");
    foo(c1);
    print!("foo2: ");
    foo(c2);
}

fn main() {
    let e = Employee::new("Hans", "Hintermeyer");

    let emp: Vec<Box<dyn Human>> = vec![Box::new(Employee::new("a", "b"))];
    let m = Manager::new("Andy", "Arbeit", emp, 1i8);

    foo(&**m.group.first().unwrap());
    foo(&m);
    foo(&e);

    foo2(&**m.group.first().unwrap(), &m);

    println!("Name is <{}>", e.full_name());
    println!("Name is <{}>", m.full_name());
}
