

pub fn calculate_factorial(num: u32)
{
    let mut array: Vec<u32> = Vec::new();
    fill_vector(num, &mut array);

    let mut numer =  num - 1;
    while numer > 1
    {
        let digit_len = number_len(numer);
        let mut items: Vec<u32> = Vec::new();
        fill_vector(numer, &mut items);
        let mut arr_help: Vec<u32> = vec![0; array.len() + (digit_len - 1)];

        let mut i = 0;
        while i < digit_len
        {
            for j in 0..array.len()
            {
                arr_help[i + j] += items[i] * array[j];
            }
            i = i + 1;
        }
        fix_vector(&mut arr_help);
        array = arr_help;
        numer = numer - 1;
    }
    print_vec(&array);
}

fn fix_vector(vec: &mut Vec<u32>)
{
    let mut i = 0;
    let mut carry = 0;

    while !(carry == 0 && i >= vec.len())
    {
        if i == vec.len()
        {
            vec.push(carry);
        }
        else
        {
            vec[i] += carry;
        }

        carry = vec[i] / 10;
        vec[i] -= carry * 10;
        i += 1;
    }
}

fn print_vec(vec: &Vec<u32>)
{
    for i in (0..vec.len()).rev()
    {
        print!("{:?}", vec[i]);
    }
    println!("");
}

fn fill_vector(mut num: u32, vec: &mut Vec<u32>)
{
    let dec = 10;
    let size = number_len(num);

    for _ in 0..size
    {
        vec.push(num % dec);
        num = num / dec;
    }
}

fn number_len(number: u32) -> usize
{
    let mut lenght: usize = 0;
    let mut num = number;

    while num != 0
    {
        num = num / 10;
        lenght = lenght + 1;
    }
    lenght
}
