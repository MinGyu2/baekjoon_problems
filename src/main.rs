fn main() {
    // let mut s = String::new();
    // std::io::&stdin().read_to_string(&mut s).expect("fail");
    baek1535_v2();
    let test = String::new();
    test.len();
    let s = 10;
}
fn baek1535_v2(){
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("fail");
    let number = number.trim().parse::<usize>().unwrap();

    let mut p = String::new();
    std::io::stdin().read_line(&mut p).expect("fail");
    let mut p = p.split(' ').map(|s| s.trim().parse::<i32>().unwrap());

    let mut h = String::new();
    std::io::stdin().read_line(&mut h).expect("fail");
    let mut h = h.split(' ').map(|s| s.trim().parse::<i32>().unwrap());

    const MAX_PHYSICAL:usize = 100;

    let mut happy = vec![0;MAX_PHYSICAL + 1];

    for _ in 1 ..= number {
        let h = h.next().unwrap();
        let pi = p.next().unwrap();
        for p in (pi as usize ..= MAX_PHYSICAL).rev() {
            happy[p] = if (p as i32) - pi > 0 { happy[p].max( happy[p - pi as usize] + h) }
                          else { happy[p] };
        }
    }
    println!("{}",happy[MAX_PHYSICAL])
}
// fn baek1535(){
//     let mut number = String::new();
//     std::io::stdin().read_line(&mut number).expect("fail");
//     let number = number.trim().parse::<usize>().unwrap();

//     let mut p = String::new();
//     std::io::stdin().read_line(&mut p).expect("fail");
//     let mut p = p.split(' ').map(|s| s.trim().parse::<i32>().unwrap());

//     let mut h = String::new();
//     std::io::stdin().read_line(&mut h).expect("fail");
//     let mut h = h.split(' ').map(|s| s.trim().parse::<i32>().unwrap());

//     const MAX_PHYSICAL:usize = 100;

//     let mut happy = vec![vec![0;MAX_PHYSICAL + 1]; (number+1) as usize];

//     for i in 1 ..= number {
//         let h = h.next().unwrap();
//         let pi = p.next().unwrap();
//         for p in 0 ..= MAX_PHYSICAL {
//             happy[i][p] = if (p as i32) - pi > 0 { happy[i-1][p].max( happy[i-1][p - pi as usize] + h) }
//                           else { happy[i-1][p] };
//         }
//     }
//     println!("{}",happy[number][MAX_PHYSICAL])
// }
// fn baek2629_v6() {
//     let mut number_of_weight = String::new();
//     std::io::stdin().read_line(&mut number_of_weight).expect("fail");
//     let number_of_weight = number_of_weight.trim().parse::<usize>().unwrap();

//     let mut weights = String::new();
//     std::io::stdin().read_line(&mut weights).expect("fail");
//     let weights:Vec<_> = weights.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();

//     let mut number_of_bead = String::new();
//     std::io::stdin().read_line(&mut number_of_bead).expect("fail");
//     let _number_of_bead = number_of_bead.trim().parse::<usize>().unwrap();

//     let mut beads_weight = String::new();
//     std::io::stdin().read_line(&mut beads_weight).expect("fail");
//     let beads_weight = beads_weight.split(' ').map(|s| s.trim().parse::<i32>().unwrap());
//     // input end

//     // let max_beads_weight:i32 = beads_weight.iter().max().unwrap() * 1 + weights.iter().max().unwrap() * 1; // 이러면 틀림 30퍼 센트 에서.
//     // let max_beads_weight:i32 = 40_000; // o
//     // let max_beads_weight:i32 = 15_000; // o
//     let max_beads_weight:i32 = weights.iter().sum(); // o
//     let mut s = vec![vec![false; (max_beads_weight + 1) as usize];2];

//     s[0][0] = true;
//     for i in 1..=number_of_weight {
//         let w = weights[i-1];
//         for bead_weight in 0 ..=max_beads_weight {
//             s[1][bead_weight as usize]
//                  = s[0][bead_weight as usize] ||
//                    s[0][i32::abs(bead_weight - w) as usize]||
//                    if bead_weight + w > max_beads_weight { false } else { s[0][(bead_weight + w) as usize] };
//         }
//         s[0] = s[1].clone();
//     }

//     for bead_weight in beads_weight {
//         if bead_weight > max_beads_weight {
//             print!("N ");
//         }else {
//             print!("{} ",
//                 if s[1][bead_weight as usize] {'Y'} else {'N'}
//             );
//         }
//     }
//     println!();
//     // println!("{:?}", s);
// }
// fn baek2629_v5() {
//     let mut number_of_weight = String::new();
//     std::io::stdin().read_line(&mut number_of_weight).expect("fail");
//     let number_of_weight = number_of_weight.trim().parse::<usize>().unwrap();

//     let mut weights = String::new();
//     std::io::stdin().read_line(&mut weights).expect("fail");
//     let weights:Vec<_> = weights.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();

//     let mut number_of_bead = String::new();
//     std::io::stdin().read_line(&mut number_of_bead).expect("fail");
//     let _number_of_bead = number_of_bead.trim().parse::<usize>().unwrap();

//     let mut beads_weight = String::new();
//     std::io::stdin().read_line(&mut beads_weight).expect("fail");
//     let beads_weight = beads_weight.split(' ').map(|s| s.trim().parse::<i32>().unwrap());
//     // input end

//     // let max_beads_weight:i32 = beads_weight.iter().max().unwrap() * 1 + weights.iter().max().unwrap() * 1; // 이러면 틀림 30퍼 센트 에서.
//     // let max_beads_weight:i32 = 40_000; // o
//     // let max_beads_weight:i32 = 15_000; // o
//     let max_beads_weight:i32 = weights.iter().sum(); // o
//     let mut s = vec![vec![false; (max_beads_weight + 1) as usize];number_of_weight+1];

//     s[0][0] = true;
//     for i in 1..=number_of_weight {
//         let w = weights[i-1];
//         for bead_weight in 0 ..=max_beads_weight {
//             s[i][bead_weight as usize]
//                  = s[i-1][bead_weight as usize] ||
//                    s[i-1][i32::abs(bead_weight - w) as usize]||
//                    if bead_weight + w > max_beads_weight { false } else { s[i-1][(bead_weight + w) as usize] };
//         }
//     }

//     for bead_weight in beads_weight {
//         if bead_weight > max_beads_weight {
//             print!("N ");
//         }else {
//             print!("{} ",
//                 if s[number_of_weight][bead_weight as usize] {'Y'} else {'N'}
//             );
//         }
//     }
//     println!();
//     // println!("{:?}", s);
// }
// fn baek2629_v4() {
//     let mut number_of_weight = String::new();
//     std::io::stdin().read_line(&mut number_of_weight).expect("fail");
//     let number_of_weight = number_of_weight.trim().parse::<usize>().unwrap();

//     let mut weights = String::new();
//     std::io::stdin().read_line(&mut weights).expect("fail");
//     let weights:Vec<_> = weights.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();

//     let mut number_of_bead = String::new();
//     std::io::stdin().read_line(&mut number_of_bead).expect("fail");
//     let _number_of_bead = number_of_bead.trim().parse::<usize>().unwrap();

//     let mut beads_weight = String::new();
//     std::io::stdin().read_line(&mut beads_weight).expect("fail");
//     let beads_weight:Vec<_> = beads_weight.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();
//     // input end

//     // let max_beads_weight:i32 = beads_weight.iter().max().unwrap() * 1 + weights.iter().max().unwrap() * 1;
//     // let max_beads_weight:i32 = 40_000;
//     let max_beads_weight:i32 = 15_000;
//     let mut s = vec![vec![false; (max_beads_weight + 1) as usize];number_of_weight+1];

//     s[0][0] = true;
//     for i in 1..=number_of_weight {
//         let w = weights[i-1];
//         for bead_weight in 0 ..=max_beads_weight {
//             s[i][bead_weight as usize]
//                  = s[i-1][bead_weight as usize] ||
//                    s[i-1][i32::abs(bead_weight - w) as usize]||
//                    if bead_weight + w > max_beads_weight { false } else { s[i-1][(bead_weight + w) as usize] };
//         }
//     }

//     for bead_weight in beads_weight {
//         if bead_weight > max_beads_weight {
//             print!("N ");
//         }else {
//             print!("{} ",
//                 if s[number_of_weight][bead_weight as usize] {'Y'} else {'N'}
//             );
//         }
//     }
//     println!();
//     // println!("{:?}", s);
// }
// fn baek2629_v3() {
//     let mut number_of_weight = String::new();
//     std::io::stdin().read_line(&mut number_of_weight).expect("fail");
//     let number_of_weight = number_of_weight.trim().parse::<usize>().unwrap();

//     let mut weights = String::new();
//     std::io::stdin().read_line(&mut weights).expect("fail");
//     let weights:Vec<_> = weights.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();

//     let mut number_of_bead = String::new();
//     std::io::stdin().read_line(&mut number_of_bead).expect("fail");
//     let _number_of_bead = number_of_bead.trim().parse::<usize>().unwrap();

//     let mut beads_weight = String::new();
//     std::io::stdin().read_line(&mut beads_weight).expect("fail");
//     let beads_weight:Vec<_> = beads_weight.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();
//     // input end

//     // let max_beads_weight:i32 = beads_weight.iter().max().unwrap() * 1 + weights.iter().max().unwrap() * 1;
//     let max_beads_weight:i32 = 40_000;
//     let mut s = vec![vec![false; (max_beads_weight + 1) as usize];number_of_weight+1];

//     s[0][0] = true;
//     for i in 1..=number_of_weight {
//         let w = weights[i-1];
//         for bead_weight in 0 ..=max_beads_weight {
//             s[i][bead_weight as usize]
//                  = s[i-1][bead_weight as usize] ||
//                    s[i-1][i32::abs(bead_weight - w) as usize]||
//                    if bead_weight + w > max_beads_weight { false } else { s[i-1][(bead_weight + w) as usize] };
//         }
//     }

//     for bead_weight in beads_weight {
//         print!("{} ",
//             if s[number_of_weight][bead_weight as usize] {'Y'} else {'N'}
//         );
//     }
//     println!();
//     // println!("{:?}", s);
// }
// fn baek2629_v2() {
//     let mut number_of_weight = String::new();
//     std::io::stdin().read_line(&mut number_of_weight).expect("fail");
//     let number_of_weight = number_of_weight.trim().parse::<usize>().unwrap();

//     let mut weights = String::new();
//     std::io::stdin().read_line(&mut weights).expect("fail");
//     let weights:Vec<_> = weights.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();

//     let mut number_of_bead = String::new();
//     std::io::stdin().read_line(&mut number_of_bead).expect("fail");
//     let _number_of_bead = number_of_bead.trim().parse::<usize>().unwrap();

//     let mut beads_weight = String::new();
//     std::io::stdin().read_line(&mut beads_weight).expect("fail");
//     let beads_weight:Vec<_> = beads_weight.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();
//     // input end

//     let max_beads_weight:i32 = beads_weight.iter().max().unwrap() * 1 + weights.iter().max().unwrap() * 1;
//     let mut s = vec![vec![false; (max_beads_weight + 1) as usize];number_of_weight+1];

//     s[0][0] = true;
//     for i in 1..=number_of_weight {
//         let w = weights[i-1];
//         for bead_weight in 0 ..=max_beads_weight {
//             s[i][bead_weight as usize]
//                  = s[i-1][bead_weight as usize] ||
//                    s[i-1][i32::abs(bead_weight - w) as usize]||
//                    if bead_weight + w > max_beads_weight { false } else { s[i-1][(bead_weight + w) as usize] };
//         }
//     }

//     for bead_weight in beads_weight {
//         print!("{} ",
//             if s[number_of_weight][bead_weight as usize] {'Y'} else {'N'}
//         );
//     }
//     println!();
//     // println!("{:?}", s);
// }
// fn baek2629() {
//     let mut number_of_weight = String::new();
//     std::io::stdin().read_line(&mut number_of_weight).expect("fail");
//     let number_of_weight = number_of_weight.trim().parse::<usize>().unwrap();

//     let mut weights = String::new();
//     std::io::stdin().read_line(&mut weights).expect("fail");
//     let mut weights = weights.split(' ').map(|s| s.trim().parse::<i32>().unwrap());
//     // let mut weights:Vec<i32> = weights.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();
//     // weights.sort();
//     // let mut weights = weights.iter();

//     let mut number_of_bead = String::new();
//     std::io::stdin().read_line(&mut number_of_bead).expect("fail");
//     let _number_of_bead = number_of_bead.trim().parse::<usize>().unwrap();

//     let mut beads_weight = String::new();
//     std::io::stdin().read_line(&mut beads_weight).expect("fail");
//     let beads_weight:Vec<_> = beads_weight.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect();
//     // input end

//     let max_beads_weight:i32 = beads_weight.iter().max().unwrap() * 1;
//     let mut s = vec![vec![false; (max_beads_weight + 1) as usize];number_of_weight+1];

//     s[0][0] = true;
//     for i in 1..=number_of_weight {
//         let w = weights.next().unwrap();
//         for bead_weight in 0 ..=max_beads_weight {
//             s[i][bead_weight as usize] = s[i-1][bead_weight as usize] || s[i-1][i32::abs(bead_weight - w) as usize];
//         }
//     }

//     for bead_weight in beads_weight {
//         print!("{} ",
//             if s[number_of_weight][bead_weight as usize] {'Y'} else {'N'}
//         );
//     }
//     println!();
// }


// fn baek9084_v2(){
//     let mut number_test_case = String::new();
//     std::io::stdin().read_line(&mut number_test_case).expect("fail");
//     let number_test_case = number_test_case.trim().parse::<usize>().unwrap();

//     for _ in 0..number_test_case {
//         let mut number_of_coin = String::new();
//         std::io::stdin().read_line(&mut number_of_coin).expect("fail coin");
//         let number_of_coin = number_of_coin.trim().parse::<usize>().unwrap();

//         let mut coins = String::new();
//         std::io::stdin().read_line(&mut coins).expect("fail coin");
//         let mut coins = coins.split(' ').map(|s| s.trim().parse::<i32>().unwrap());

//         let mut money = String::new();
//         std::io::stdin().read_line(&mut money).expect("fail money");
//         let money = money.trim().parse::<i32>().unwrap();

//         let mut s = vec![0; (money+1) as usize];
//         for _ in 1 ..= number_of_coin {
//             let c = coins.next().unwrap();
//             for m in 0 ..= money {
//                 s[m as usize] = 
//                     if m == 0 { 1 }
//                     else if m - c < 0 { s[m as usize] }
//                     else { s[m as usize] + s[(m-c) as usize] }
//             }
//         }
//         println!("{}", s[money as usize]);
//     }
// }

// fn baek9084(){
//     let mut number_test_case = String::new();
//     std::io::stdin().read_line(&mut number_test_case).expect("fail");
//     let number_test_case = number_test_case.trim().parse::<usize>().unwrap();

//     for _ in 0..number_test_case {
//         let mut number_of_coin = String::new();
//         std::io::stdin().read_line(&mut number_of_coin).expect("fail coin");
//         let number_of_coin = number_of_coin.trim().parse::<usize>().unwrap();

//         let mut coins = String::new();
//         std::io::stdin().read_line(&mut coins).expect("fail coin");
//         let mut coins = coins.split(' ').map(|s| s.trim().parse::<i32>().unwrap());

//         let mut money = String::new();
//         std::io::stdin().read_line(&mut money).expect("fail money");
//         let money = money.trim().parse::<i32>().unwrap();

//         let mut s = vec![ vec![0;(money+1) as usize];number_of_coin+1];
//         for i in 1 ..= number_of_coin {
//             let c = coins.next().unwrap();
//             for m in 0 ..= money {
//                 s[i][m as usize] = 
//                     if m == 0 { 1 }
//                     else if m - c < 0 { s[i-1][m as usize] }
//                     else { s[i-1][m as usize] + s[i][(m-c) as usize] }
//             }
//         }
//         println!("{}", s[number_of_coin][money as usize]);
//     }
// }

// fn baek7579_v3() {
//     // input N,M
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let mut input = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap());
//     let (N, M) = (input.next().unwrap(), input.next().unwrap());

//     // input m
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let m:Vec<i32> = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap()).collect();

//     // input c
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let c:Vec<i32> = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap()).collect();

//     // let sum_m:i32 = m.iter().sum();
//     let mut z = vec![vec![0;10000+1];(N+1) as usize]; // sum_m >= M
//     for j in 1..=N {
//         let (m,c) = (m[(j-1)as usize], c[(j-1) as usize]);
//         for i in 1..=10000 {   // cost
//             z[j as usize][i as usize] = if i - c < 0 {
//                 z[(j-1) as usize][i as usize]
//             }else {
//                 z[(j-1) as usize][i as usize].max(z[(j-1) as usize][(i-c) as usize] + m)
//             }
//         }        
//     }
//     // for i in 0..z[N as usize].len() {
//     //     if z[N as usize][i] >= M {
//     //         println!("{}",i);
//     //         break;
//     //     }
//     // }
//     let ans = z[N as usize].iter().enumerate().find(|(i,&m)| m>= M).unwrap().0;
//     // println!("{:?}",z );
//     println!("{}",ans);
// }
// fn baek7579_v2() {
//     // input N,M
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let mut input = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap());
//     let (N, M) = (input.next().unwrap(), input.next().unwrap());

//     // input m
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let m:Vec<i32> = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap()).collect();

//     // input c
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let c:Vec<i32> = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap()).collect();

//     let sum_m:i32 = m.iter().sum();
//     let mut z = vec![0;(sum_m - M + 1) as usize]; // sum_m >= M
//     for j in 1..=N {
//         let (m,c) = (m[(j-1)as usize], c[(j-1) as usize]);
//         for i in (m..=(sum_m - M)).rev() {   
//             z[i as usize] = z[i as usize].max(z[(i-m) as usize] + c)
//         }        
//     }
//     println!("{}", c.iter().sum::<i32>() - z[(sum_m - M) as usize])
// }
// fn baek7579() {
//     // input N,M
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let mut input = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap());
//     let (N, M) = (input.next().unwrap(), input.next().unwrap());

//     // input m
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let m:Vec<i32> = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap()).collect();

//     // input c
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("fail");
//     let c:Vec<i32> = input.split(' ').map(|x| x.trim().parse::<i32>().unwrap()).collect();

//     let sum_m:i32 = m.iter().sum();
//     let mut z = vec![vec![0; (N+1) as usize];(sum_m - M + 1) as usize]; // sum_m >= M
//     for j in 1..=N {
//         let (m,c) = (m[(j-1)as usize], c[(j-1) as usize]);
//         for i in 1..=(sum_m - M) {   
//             z[i as usize][j as usize] = if i-m < 0 {
//                 z[i as usize][(j-1) as usize]
//             }else {
//                 z[i as usize][(j-1) as usize].max(z[(i-m) as usize][(j-1) as usize] + c)
//             }
//         }        
//     }
//     // println!("{}", z[(sum_m - M) as usize][N as usize]);
//     println!("{}", c.iter().sum::<i32>() - z[(sum_m - M) as usize][N as usize])
//     // for i in m.iter().zip(c.iter()){
//     //     println!("{:?}", i);
//     // }
//     // println!("{} {}",n,m);
//     // println!("{:?} {:?}",a,c);
//     // println!("{:?}",z);
// }
// fn baek12865_v3(){
//     let mut s = String::new();
//     std::io::stdin().read_line(&mut s).expect("fail");
//     let mut v = s.split(' ').map(|i| -> i32 { i.trim().parse::<i32>().expect("fail parse") });
//     let (n, k) = (v.next().unwrap(), v.next().unwrap());
    
//     let mut dp = vec![0;(k+1) as usize];
//     // let mut dp = vec![vec![0;10];14];
//     for _ in 1 ..= n { // 현재 받은 물건 무개 + 가치
//         let mut s = String::new();
//         std::io::stdin().read_line(&mut s).expect("fail");
//         let mut v = s.split(' ').map(|i| -> i32 { i.trim().parse::<i32>().expect("fail parse")});
//         let (w,v) = (v.next().unwrap(), v.next().unwrap());

//         for i in  (w..= k).rev() { // 무게
//             dp[i as usize] = dp[i as usize].max(dp[(i-w) as usize ] + v)
//         }
//     }
//     // dp[k as usize][n as usize] = 10;
//     println!("{}",dp[k as usize]);
//     // println!("{:?}", dp)
// }
// fn baek12865_v2(){
//     let mut s = String::new();
//     std::io::stdin().read_line(&mut s).expect("fail");
//     let mut v = s.split(' ').map(|i| -> i32 { i.trim().parse::<i32>().expect("fail parse") });
//     let (n, k) = (v.next().unwrap(), v.next().unwrap());
    
//     let mut dp = vec![vec![0;101];100_001];
//     // let mut dp = vec![vec![0;10];14];
//     for j in 1 ..= n { // 현재 받은 물건 무개 + 가치
//         let mut s = String::new();
//         std::io::stdin().read_line(&mut s).expect("fail");
//         let mut v = s.split(' ').map(|i| -> i32 { i.trim().parse::<i32>().expect("fail parse")});
//         let (w,v) = (v.next().unwrap(), v.next().unwrap());
//         for i in 1 ..= k { // 무게
//             dp[i as usize][j as usize] = if i - w < 0 {
//                 dp[i as usize][(j-1) as usize]
//             }else {
//                 dp[i as usize][(j-1) as usize].max(dp[(i-w) as usize ][(j-1) as usize] + v)
//             }
//         }
//     }
//     // dp[k as usize][n as usize] = 10;
//     println!("{}",dp[k as usize][n as usize]);
// }
// fn baek12865(){
//     let mut s = String::new();
//     std::io::stdin().read_line(&mut s).expect("fail");
//     let mut v = s.split(' ').map(|i| -> i32 { i.trim().parse::<i32>().unwrap() });
//     let (n,k) = (v.next().unwrap(), v.next().unwrap());

//     // let object = &mut [(0,0);100];
//     let mut object = Vec::<(i32,i32)>::new();
//     for i in 0..n {
//         let mut s = String::new();
//         std::io::stdin().read_line(&mut s).expect("fail");
//         let mut v = s.split(' ').map(|i| -> i32 { i.trim().parse::<i32>().unwrap() });
//         object.push((v.next().unwrap(), v.next().unwrap()));
//         // if let Some(item) = object.get_mut(i as usize) {
//         //     *item = (v.next().unwrap(), v.next().unwrap());
//         // }
//     }
//     // input end

//     println!("{:?}",object);

//     // object.sort_unstable_by(|a,b| a.0.cmp(&b.0));
//     let mut all_of_answers = Vec::<(i32,i32)>::new();
//     all_of_answers.push((0,0));
    
//     for ori in &object {
//         for i in 0..all_of_answers.len() {
//             if ori.0 + all_of_answers[i].0 > k { continue; }
//             all_of_answers.push((ori.0 + all_of_answers[i].0 , ori.1 + all_of_answers[i].1));
//         }
//     }
//     let all_of_answers= all_of_answers
//         .iter()
//         // .filter(|i| -> bool { i.0 <= k })
//         .max_by(|a,b| a.1.cmp(&b.1))
//         .unwrap().1;
//     println!("{}", all_of_answers);
// }
// // fn test2() {
// //     let mut s = String::new();
// //     std::io::stdin().read_line(&mut s).expect("fail");
// //     let z:Vec<&str> = s.split(' ').collect();
// //     println!("{:?}", z);
// //     let s = s.split(' ').map(|x| -> i32 { x.trim().parse().unwrap()});
// //     let mut v:Vec<i32> = s.collect();
// //     // println!("{:?}", v);
// //     // v.dedup();
// //     // v.group_by(|a,b| a >= b );
// //     // v.dedup_by(|a,b| -> bool { a == b});
// //     v.sort_unstable_by(|a,b| -> Ordering { b.cmp(a) });
// //     v.dedup();
// //     println!("{:?}", v);
// // }
// // fn test() {
// //     let mut v = [3,2,5,2,1];
// //     let mut s = ["test", "df","asfadd"];
// //     s.sort_by(|a,b| -> Ordering { a.len().cmp(&b.len() ) } );
// //     v.select_nth_unstable(0);
// //     println!("{:?}", v);
// //     println!("{:?}", s);
// // }
// fn baek2108() {
//     // input start
//     let mut n = String::new();
//     std::io::stdin().read_line(&mut n).expect("fail read line");
//     let n:usize = n.trim().parse().expect("fial str to usize");
//     let mut vec:Vec<i32> = Vec::new();
//     for _ in 0..n {
//         let mut value = String::new();
//         std::io::stdin().read_line(&mut value).expect("fail read value");
//         let value:i32 = value.trim().parse().expect("fail string to i32");
//         vec.push(value);
//     }
//     // input end

//     // 산술 평균
//     let mean = (vec.iter().sum::<i32>() as f64 / n as f64).round() as i32;
//     println!("{}",mean);

//     // 중앙값
//     vec.sort();
//     println!("{}",vec[vec.len()/2]);

//     // 최빈값
//     let mut min_v1 = 4001;
//     let mut min_cnt = 0;
//     let mut vec_freq = Vec::<i32>::new();
//     let mut vec_only_freq = Vec::<i32>::new();
//     for i in &vec {
//         if min_v1 != *i {
//             if min_v1 != 4001 {
//                 vec_freq.push(min_cnt);
//                 vec_only_freq.push(min_cnt);
//             }
//             vec_freq.push(*i);
//             min_cnt = 1;
//         }else {
//             min_cnt +=1;
//         }
//         min_v1 = *i;
//     }
//     vec_freq.push(min_cnt);
//     vec_only_freq.push(min_cnt);

//     vec_only_freq.sort();
//     let max_cnt = vec_only_freq[vec_only_freq.len() - 1];

//     let mut cnt = 0;
//     let mut freq_value = 0;
//     for i in 0..vec_only_freq.len() {
//         if max_cnt == vec_freq[2*i+1] {
//             freq_value = vec_freq[2*i];
//             cnt += 1;
//             if cnt == 2 { break; }
//         }
//     }
//     // println!("{:?}", vec_freq);
//     // println!("{:?}", vec_only_freq);
//     println!("{}",freq_value);
//     // 범위
//     println!("{}", vec[vec.len() - 1] - vec[0]);
// }

// fn baek25421() {
//     let mut n = String::new();
//     std::io::stdin().read_line(&mut n).expect("fail read line");
//     let mut n :i32= n.trim().parse().expect("fail str to int");

//     let mut f:[u128;9] = [1;9];
//     let mut t:[u128;9] = [0;9];

//     if n == 1 {
//         println!("9");
//         return 
//     }
//     while n-1 > 0 {
//         n -=1;
//         // println!("{}",n);

//         for i in 0..9 {
//             let start = if i < 2 { 0 } else { i-2 } as usize;
//             let end = if i+2 > 8 { 9 } else { i+2 +1 } as usize;
//             t[i] = f[start..end].iter().sum::<u128>() % 987_654_321;
//             // println!("{}",i);
//         }
//         f = t;
//         t = [0;9];
//     }
//     println!("{}", f.iter().sum::<u128>() % 987_654_321)
// }

// fn baek1002() {
//     let mut t = String::new();
//     std::io::stdin().read_line(&mut t).expect("fail read line");
//     let t = match t.trim().parse::<i32>() {
//         Ok(n) => n,
//         _ => 0
//     };
//     for _ in 0..t {
//         let mut i = String::new();
//         std::io::stdin().read_line(&mut i).expect("fail read line");
//         let v:Vec<&str> = i.split(' ').collect();
//         // let qq:Vec<i32> = Vec::new();
//         let (x1,y1,r1) 
//             = (str_to_i32(v[0]), str_to_i32(v[1]), str_to_i32(v[2]));
//         let (x2, y2, r2)
//             = (str_to_i32(v[3]), str_to_i32(v[4]), str_to_i32(v[5]));
        
//         let max = r1.max(r2);
//         let min = r1.min(r2);

//         let z = (x1 - x2).pow(2) + (y1 - y2).pow(2);
//         // let j = i.split_whitespace().map(|i| i.trim().parse::<i32>());
//         let sol = if z > max.pow(2) {
//             match z.cmp(&(max + min).pow(2)) {
//                 Ordering::Greater => 0,
//                 Ordering::Equal => 1,
//                 Ordering::Less => 2
//             }
//         }else {
//             match z.cmp(&(max - min).pow(2)) {
//                 Ordering::Less => 0,
//                 Ordering::Equal => if z==0 { -1 } else { 1 },
//                 Ordering::Greater => 2
//             }
//         };
//         println!("{}",sol);
//     }
// }

// fn str_to_i32(str:&str) -> i32 {
//     match str.trim().parse::<i32>() {
//         Ok(n) => n,
//         _ => 0
//     }
// }

// fn baek1001() {
//     let mut str = String::new();
//     std::io::stdin().read_line(&mut str).expect("error");

//     let z = str.split(' ');
//     let vec:Vec<&str> = z.collect();
//     let sum = match vec[0].trim().parse::<i32>() {
//         Ok(n) => n,
//         Err(_) => 0
//     } - match vec[1].trim().parse::<i32>() {
//         Ok(n) => n,
//         Err(_) => 0
//     };
//     println!("{}",sum);
// }