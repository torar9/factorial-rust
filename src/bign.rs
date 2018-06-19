pub fn calculate_factorial(num: u32)
{
    let digit_len = number_len(num);
    println!("digit_len: {:?}", digit_len);
}

fn number_len(number: u32) -> u32
{
    let mut lenght: u32 = 0;
    let mut num = number;
    
    while num != 0
    {
        num = num / 10;
        lenght = lenght + 1;
    }
    lenght
}
