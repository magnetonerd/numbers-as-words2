fn num_word(num: i32) -> &'static str{
    let name = match num{
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => "FUCK OFF YOU!",   
    };
    return name;
    panic!("WHAT DID YOU DO!?");
}

fn num_word_less100(num: i32) -> String{
    if ((num < 100) && (num > 9)){ 
        let mut string = match num/10{
            1 => num_word(10).to_string(),
            2 => num_word(20).to_string(),
            3 => num_word(30).to_string(),
            4 => num_word(40).to_string(),
            5 => num_word(50).to_string(),
            6 => num_word(60).to_string(),
            7 => num_word(70).to_string(),
            8 => num_word(80).to_string(),
            9 => num_word(90).to_string(),
            _ => "WHY ARE YOUD DOING THIS TO ME?!".to_string(),
        };
        if (num % 10 > 0){
            string.push_str(num_word(num % 10));
        }
        return string;
    }else{
        println!("DO YOU EVEN KNOW WHAT YOU ARE DOING?!");
    }
    panic!("YOU FUCKED UP NUM_WORD_LESS100!!!!!!");
}

fn num_word_greater100(num: i32) -> String{
    if ((num >= 100) && (num < 1000)){ 
        let mut string = match num/100{
            1 => num_word(1).to_string(),
            2 => num_word(2).to_string(),
            3 => num_word(3).to_string(),
            4 => num_word(4).to_string(),
            5 => num_word(5).to_string(),
            6 => num_word(6).to_string(),
            7 => num_word(7).to_string(),
            8 => num_word(8).to_string(),
            9 => num_word(9).to_string(),
            _ => "WHY ARE YOUD DOING THIS TO ME?!".to_string(),
        };
        string.push_str("hundred");
        let mut newnum1 = num/100;
        newnum1 = newnum1*100;
        let mut newnum2 = num - newnum1;
        if ((newnum2 > 0) && (newnum2 < 20)){
            string.push_str("and");
            string.push_str(num_word(newnum2));
        }
        if ((newnum2 >= 20) && (newnum2 < 100)){
            string.push_str("and");
            match newnum2/10{
                2 => string.push_str(num_word(20)),
                3 => string.push_str(num_word(30)),
                4 => string.push_str(num_word(40)),
                5 => string.push_str(num_word(50)),
                6 => string.push_str(num_word(60)),
                7 => string.push_str(num_word(70)),
                8 => string.push_str(num_word(80)),
                9 => string.push_str(num_word(90)),
                _ => string.push_str("WHY ARE YOUD DOING THIS TO ME?!"),
            };
            if(newnum2 % 10 > 0){
                string.push_str(num_word(newnum2 % 10));
            }
        }
        return string;
    }else{
        println!("DO YOU EVEN KNOW WHAT YOU ARE DOING?!");
    }
    panic!("YOU FUCKED UP NUM_WORD_GREATER100!!!!!!");
}

fn main() {
    let mut num = 0;
    let mut total = 0;
    for num in 1..1001{
        if (num < 20){
            total = total + num_word(num).len();
            //println!("{}    ,{}    ,{},    {}",num,num_word(num),num_word(num).len(),total);
        }else if ((num >= 20) && (num < 100)){
            total = total + num_word_less100(num).len();
            //println!("{}    ,{}    ,{}    ,{}",num,num_word_less100(num),num_word_less100(num).len(),total);
        }else if ((num >= 100) && (num < 1000)){
            total = total + num_word_greater100(num).len();
            //println!("{}    ,{}    ,{}    ,{}",num,num_word_greater100(num),num_word_greater100(num).len(),total);
        }else if (num == 1000){
            total = total + "onethousand".len();
            //println!("{}    ,{}    ,{}    ,{}",num,"onethousand","onethousand".len(),total);
        }
    }
    println!("{}",total);
}
