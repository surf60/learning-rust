fn main(){
        // variable to store integer value
        // mut makes it mutable
        let mut age = 31;
        println!("Age: {}", age);
    
        // variable to store floating-point value
        let salary = 342523.23;
        println!("Salary: {}", salary);
    
        // variable to store string
        let name = "Jackie";
        println!("Name: {}", name);

        age = 500;
        println!("Age: {}", age);

        const PI: f32 = 3.14;
        print!("PI is: {}",PI)
}