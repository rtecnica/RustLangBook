/*
 * Using a hash map and vectors, create a text interface to allow a user to
 * add employee names to a department in a company. For example, “Add
 * Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
 * a list of all people in a department or all people in the company by
 * department, sorted alphabetically.
 *
 * Data tree should look like:
 * - Company1
 *      - Department1
 *          - Employee1
 *          - Employee2
 *          - Employee3
 *      - Department2
 *          - Employee4
 *          - Employee5
 *          - Employee6
 *      - Department3
 *          - Employee7
 * - Company2
 *      [...]
 *
 *
 *
 */

use std::io;

mod hr;

fn main() {
    let mut company = hr::Company::new(String::from("Company Inc."));


    loop {
        let mut unparsed_cmd = String::new();
        io::stdin()
            .read_line(&mut unparsed_cmd)
            .expect("Problem reading line!");
        unparsed_cmd.pop();
        unparsed_cmd.push(' ');
        let mut iter = unparsed_cmd.split(' ');
        match iter.next().unwrap() {
            "add" => {
                let emp_s = iter.next().unwrap();
                iter.next();
                let dep_s = iter.next().unwrap();

                company.add_emp(
                    hr::Department::new(String::from(dep_s)),
                    hr::Employee::new(String::from(emp_s)),
                );
            }
            "list" => company.print(),
            "quit" => break,
            s => println!("Didn't understand command: {}", s),
        }
    }
}
