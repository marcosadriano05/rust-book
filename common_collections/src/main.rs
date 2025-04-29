use std::{collections::HashMap, io};

mod median;
mod mode;
mod pig_latin;
mod sorting;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Add an employee to a departament!");
    loop {
        println!("====================");
        println!("What do you want?");
        println!("1 - Add an employee to a departament.");
        println!("2 - List all employees in a departament.");
        println!("3 - Exit.");
        println!("====================");
        let mut employee = String::new();
        io::stdin()
            .read_line(&mut employee)
            .expect("Fail to read employee");

        let mut departament = String::new();
        io::stdin()
            .read_line(&mut departament)
            .expect("Fail to read departament");

        let comp: &mut Vec<String> = company.entry(departament).or_insert(Vec::new());
        comp.push(employee);
        println!("{:?}", company);
    }
}
