use std::io;
use std::process::exit;

enum Sex {
    M,
    F,
}


fn main() {
    println!("          CALORIE CALCULATOR");

   let bmr: f32 = calculate_bmr();
   calculate_clories(bmr);
    
}

fn calculate_bmr() -> f32
{
    println!("Please, enter the following data to calculate your BMR");

    println!("Sex (m/f): ");
    let mut sex_input = String::new();
    io::stdin().read_line(&mut sex_input).expect("failed to read line");
    let sex_input: char = sex_input.trim().parse().expect("Invalid character");

    let sex : Sex;
    if sex_input == 'm' || sex_input == 'M'{
        sex = Sex::M;
    }
    else if sex_input == 'f' || sex_input == 'F'{
        sex = Sex::F;
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

    let bmr: f32;
    if let Sex::M = sex {
        bmr = 9.99 * weight + 6.25 * height - 4.92 * age + 5.0;
    }
    else {
        bmr = 9.99 * weight + 6.25 * height - 4.92 * age - 161.0;
    }

    println!("Your BMR is: {bmr}");
    return bmr;
}

fn calculate_clories(bmr: f32)
{
    println!("Activity level (1-6): ");
    let mut activity = String::new();
    io::stdin().read_line(&mut activity).expect("failed to read line");
    let activity: i32 = activity.trim().parse().expect("please type a number 1-6");

    let activity_multiplier : f32;

    match activity {
        1 => activity_multiplier = 1.2,
        2 => activity_multiplier = 1.4,
        3 => activity_multiplier = 1.6,
        4 => activity_multiplier = 1.75,
        5 => activity_multiplier = 2.0,
        6 => activity_multiplier = 2.4,
        _ => {
            println!("please type a number 1-6");
            exit(1);
        }
    }

    let calories : f32 = bmr * activity_multiplier;
    println!("Your calorie deficit is {} calories", calories);
}
