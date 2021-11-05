/*
 * Parse commands from interface
 *
 *
 */

struct Command {
    name: String,
    action: fn(Vec<String>) -> (),
}

impl Command {
    fn new(name: String, action: fn(Vec<String>) -> ()) -> Command {
        Command { name, action }
    }

    fn act(&self, args: Vec<String>) {
        (self.action)(args);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn print(args: Vec<String>) {
        for a in args {
            println!("{}", a);
        }
    }

    #[test]
    fn basics() {
        let cmd = Command::new(String::from("command"), print);
        let args = vec![String::from("potato"), String::from("tomato"), String::from("asdasd")];

        cmd.act(args);
    }
}
