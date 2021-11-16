/*
 * HR collections of functions
 *
 * Based on object Company with underlying tree
 *
 * Methods for editing:
 *  - Add
 *  - Remove
 *  - Peek
 */

pub struct Company {
    name: String,
    dep: Vec<Department>,
}

impl Company {
    pub fn new(name: String) -> Company {
        Company { name, dep: vec![] }
    }

    pub fn check_dep(&self, dep: &Department) -> bool {
        for x in &self.dep[..] {
            if x.name == dep.name {
                return true;
            }
        }
        false
    }

    pub fn add_dep(&mut self, dep: Department) {
        if let false = self.check_dep(&dep) {
            self.dep.push(dep);
        }
    }

    pub fn add_emp(&mut self, dep: Department, emp: Employee) {
        if let false = self.check_dep(&dep) {
            self.add_dep(Department::new(String::from(&dep.name))); 
        }
        for x in &mut self.dep {
            if x.name == dep.name {
                 x.add_emp(Employee::new(String::from(&emp.name))) 
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.name);
        for d in &self.dep {
            d.print();
        }
    }
}

pub struct Department {
    name: String,
    emp: Vec<Employee>,
}

impl Department {
    pub fn new(name: String) -> Department {
        Department { name, emp: vec![] }
    }

    pub fn check_emp(&self, emp: &Employee) -> bool {
        for x in &self.emp[..] {
            if x.name == emp.name {
                return true;
            }
        }
        false
    }

    pub fn add_emp(&mut self, emp: Employee) {
        if let false = self.check_emp(&emp) {
            self.emp.push(emp);
        }
    }

    pub fn print(&self) {
        println!("\t{}", self.name);
        for e in &self.emp {
            e.print();
        }
    }
}

pub struct Employee {
    name: String,
}

impl Employee {
    pub fn new(name: String) -> Employee {
        Employee { name }
    }

    pub fn print(&self) {
        println!("\t\t{}", self.name);
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_and_check() {
        let mut company = Company::new(String::from("Company"));
        let mut department = Department::new(String::from("Department"));
        let mut department2 = Department::new(String::from("Department"));
        let employee = Employee::new(String::from("Employee"));
        let employee2 = Employee::new(String::from("Potato"));
        company.add_emp(department2, employee2);
        company.add_emp(department, employee);
        company.print();
        assert_eq!("Company", company.name);
        assert_eq!("Department", company.dep[0].name);
        assert_eq!("Employee", company.dep[0].emp[0].name);
    }
}
