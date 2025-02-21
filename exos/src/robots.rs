use robot::Instruction;
use robot::Robot;
use robot::*;

fn main() {
    //TODO create a variable <robot> containing a new robot.
    println!("{}", robot);
    robot.process(Instruction::Print(String::from("toto")));
    println!("{}", robot);
    robot.process(Instruction::Clear);
    println!("{}", robot);
    let v = vec![
        String::from("toto"),
        String::from("toolong"),
        String::from("tata"),
        String::from("titi"),
    ];
    robot.process(Instruction::PrintVector(v));
    println!("{}", robot);
    //TODO reset robot and print result
}
