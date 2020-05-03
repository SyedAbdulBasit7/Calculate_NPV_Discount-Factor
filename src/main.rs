use std::io;
fn main() {
    let mut discount_factor=String::new();
    let mut years=String::new();
    let mut sum:f32=0.0;
    println!("Enter Discount Factor:");
    io::stdin().read_line(&mut discount_factor).expect("Failed to read line");
    let discount_factor:f32=discount_factor.trim().parse().unwrap();
    println!("Enter Years:");
    io::stdin().read_line(&mut years).expect("Failed to read line");
    let years:i32=years.trim().parse().unwrap();
    println!("===========================================================");
    println!("Years = {} Discount Factor = {}",years,discount_factor);
    for i in 1..=years{
        let mut calculate_df=1 as f32/((1 as f32+discount_factor).powf(i as f32));
        sum = sum + calculate_df;
        println!("df = {}",calculate_df);
    }
    println!("Total Discount Factor = {}",sum);

}
