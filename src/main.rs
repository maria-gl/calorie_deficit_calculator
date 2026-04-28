use std::io;
use std::process::exit;

fn main() {
    println!("          CALORIE CALCULATOR");

    println!("Please, enter the following data to calculate your BMR");

    println!("Sex (m/f): ");
    let mut sex = String::new();
    io::stdin().read_line(&mut sex).expect("failed to read line");
    let sex: char = sex.trim().parse().expect("please enter a valid value. Valid values: m, f");
    if sex == 'm' || sex == 'M'{
        println!("M!");
    }
    if sex == 'f' || sex == 'F'{
        println!("F!");
    }
    else {
        println!("Invalid value. Valid values: m, f");
        exit(1);
    }

    println!("Age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read line");
    let age: f32 = age.trim().parse().expect("please type a number");

    println!("Weight (kg): ");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("failed to read line");
    let weight: f32 = weight.trim().parse().expect("please type a number");

    println!("Height (cm): ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("failed to read line");
    let height: f32 = height.trim().parse().expect("please type a number");

    if sex == 'm' || sex == 'M'{
        let bmr: f32 = 9.99 * weight + 6.25 * height - 4.92 * age + 5.0;
    }
    else {
        let bmr: f32 = 9.99 * weight + 6.25 * height - 4.92 * age + 161.0;
    }

    println!("Your BMR is: {bmr}")
    
}
