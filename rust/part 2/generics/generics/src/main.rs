struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records{
    employees: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() != 0{
            let result = self.employees[0].name.clone();
            self.employees.remove(0);
            Some(result)
        }else {
            None
        }
    }
}

fn main() {

    let emp_1 = Employee{
        name: "George".to_string(),
        salary: 7200,
    };
    let emp_2 = Employee{
        name: "Anna".to_string(),
        salary: 8200,
    };

    let mut emp3 = Employee_Records{employees: vec![emp_1, emp_2]};

    for employee in &mut emp3 {
        println!("{:?}", employee);
    }

    // println!("{:?}", emp3.next());
    // println!("{:?}", emp3.next());
    // println!("{:?}", emp3.next());

}