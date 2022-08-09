use std::cmp::Ordering;

fn main() {
    baek25421();
}

fn baek25421() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("fail read line");
    let mut n :i32= n.trim().parse().expect("fail str to int");

    let mut f:[u128;9] = [1;9];
    let mut t:[u128;9] = [0;9];

    if n == 1 {
        println!("9");
        return 
    }
    while n-1 > 0 {
        n -=1;
        // println!("{}",n);

        for i in 0..9 {
            let start = if i < 2 { 0 } else { i-2 } as usize;
            let end = if i+2 > 8 { 9 } else { i+2 +1 } as usize;
            t[i] = f[start..end].iter().sum::<u128>() % 987_654_321;
            // println!("{}",i);
        }
        f = t;
        t = [0;9];
    }
    println!("{}", f.iter().sum::<u128>() % 987_654_321)
}

fn baek1002() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("fail read line");
    let t = match t.trim().parse::<i32>() {
        Ok(n) => n,
        _ => 0
    };
    for _ in 0..t {
        let mut i = String::new();
        std::io::stdin().read_line(&mut i).expect("fail read line");
        let v:Vec<&str> = i.split(' ').collect();
        // let qq:Vec<i32> = Vec::new();
        let (x1,y1,r1) 
            = (str_to_i32(v[0]), str_to_i32(v[1]), str_to_i32(v[2]));
        let (x2, y2, r2)
            = (str_to_i32(v[3]), str_to_i32(v[4]), str_to_i32(v[5]));
        
        let max = r1.max(r2);
        let min = r1.min(r2);

        let z = (x1 - x2).pow(2) + (y1 - y2).pow(2);
        // let j = i.split_whitespace().map(|i| i.trim().parse::<i32>());
        let sol = if z > max.pow(2) {
            match z.cmp(&(max + min).pow(2)) {
                Ordering::Greater => 0,
                Ordering::Equal => 1,
                Ordering::Less => 2
            }
        }else {
            match z.cmp(&(max - min).pow(2)) {
                Ordering::Less => 0,
                Ordering::Equal => if z==0 { -1 } else { 1 },
                Ordering::Greater => 2
            }
        };
        println!("{}",sol);
    }
}

fn str_to_i32(str:&str) -> i32 {
    match str.trim().parse::<i32>() {
        Ok(n) => n,
        _ => 0
    }
}

fn baek1001() {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).expect("error");

    let z = str.split(' ');
    let vec:Vec<&str> = z.collect();
    let sum = match vec[0].trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0
    } - match vec[1].trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0
    };
    println!("{}",sum);
}