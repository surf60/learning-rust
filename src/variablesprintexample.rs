fn main(){
    let age = 31;
    let name = "surf";
    // printting a variable
    print!("{}",age);
    // "{}" is a placeholder
    println!("{1}, age = {0}", name, age);
    // each variable replaces placeholder x in sucsession unless specified
    println!("name = {}\n age = {}",name,age);
}