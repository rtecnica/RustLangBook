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

mod hr;
mod command;
fn main() {
    
    let company = hr::Company::new(String::from("Company Inc."));

    let commands = vec![command::Command::new(String::from("add"), 

                                              |s: Vec<String>| {}),
                        command::Command::new(String::from("list"), 
                                              |s: Vec<String>| {}),
                        command::Command::new(String::from("remove"), 
                                              |s: Vec<String>| {})];
    
    let parsed_command = String::from("add potato to vegetables");
    
}
