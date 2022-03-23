use proconio::{fastout, input};
 
#[fastout]
fn main(){
    input! {
        n:usize,
        p:[usize; n],
    }
    let mut max = p[0];
    let mut counter = 0;
    for i in 0..n{
        if max < p[i] {
            max = p[i];
        } else {
            counter += 1;
        }
    }
    println!("{}", counter);
    
}



use proconio::{fastout, input};


#[fastout]
fn main(){
    input! {
        n:usize,
        k:usize,
        mut h:[isize; n],
    }use proconio::{fastout, input, marker::Chars};
    use std::collections::*;
    use std::collections::HashSet;
    use std::cmp;
    use core::cmp::*;
    use std::io;
    use num::integer::lcm;
    use libm::atan;
    use std::convert::TryInto;
    use num::integer::gcd;
    use num::integer::sqrt;
    use std::boxed::Box;
    use std::collections::VecDeque;
    
    fn main() {
        input! {
            n:usize,
            m:usize,
            a:[usize; n],
            b:[usize; m],
        }
        let mut aa = HashMap::new();
        let mut bb = HashMap::new();
        for x in a{
            let counter = aa.entry(x).or_insert(0);
            *counter += 1;
        }
        for x in b{
            let counter = bb.entry(x).or_insert(0);
            *counter += 1;
        }
        for (key, value) in bb{
            if aa.contains_key(&key){
                let value2 = aa.get(&key) as usize;
                aa.insert(key, (value2-1) as String);
                if aa.get(&key) == 0{
                    aa.remove(&key);
                }
            } else {
                println!("No");
                return;
            }use proconio::{fastout, input, marker::Chars};
            use std::collections::*;
            use std::collections::HashSet;
            use std::cmp;
            use core::cmp::*;
            use std::io;
            use num::integer::lcm;
            use libm::atan;
            use std::convert::TryInto;
            use num::integer::gcd;
            use num::integer::sqrt;
            use std::boxed::Box;
            use std::collections::VecDeque;
            
            fn main() {
                input! {
                    n:usize,
                    m:usize,
                    a:[usize; n],
                    b:[usize; m],
                }
                let mut aa = HashMap::new();
                let mut bb = HashMap::new();
                for x in a{
                    let counter = aa.entry(x).or_insert(0);
                    *counter += 1;
                }
                for x in b{
                    let counter = bb.entry(x).or_insert(0);
                    *counter += 1;
                }
                for (key, value) in bb{
                    if aa.contains_key(&key){
                        let value2 = aa.get(&key) as usize;
                        aa.insert(key, (value2-1) as String);
                        if aa.get(&key) == 0{
                            aa.remove(&key);
                        }
                    } else {
                        println!("No");
                        return;
                    }
                }
                println!("Yes");
            }
        }
        println!("Yes");
    }
        if len 
    }    
}


use proconio::{fastout, input};
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n:usize,
        xy:[[f64; 2]; n],
    }
    let mut ans:f64;
    let a = 0;
    let b = 0;

    for x in (0..n).permutations(n){
        for i in 0..n-1{
            a = x[i];
            b = x[i+1];
        }

        let dx:f64 = xy[0][a] - xy[0][b];
        let dy:f64 = xy[1][a] - xy[1][b];

        ans += (dx * dx + dy * dy).powf(2.0);
    }

    let i:f64 = n as f64;
    println!("{:..9}", ans/i);
}

use proconio::{fastout, input};
use std::collections::HashSet;
use core::str::Chars;
use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;
use num::integer::gcd;
use num::integer::sqrt;
use std::boxed::Box;
use std::collections::VecDeque;

fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }
    let mut tmp = a[0];
    let mut counter = 1;
    let mut que = Vec::new();
    let mut x = 0;
    que.push(a[0]);
    println!("1");

    for i in 1..n{
        que.push(a[i]);
        
        if tmp == a[i]{
            counter += 1;
            if counter == a[i]{
                for j in 0..counter{
                    que.pop();
                }
                counter = 1;
                x = a[i];
            }
        } else {
            counter = 1;
        }
        println!("{}", que.len());
        if que.len() != 0{
            tmp = que[que.len()-1];
        } else {
            tmp = 0;
        }
        x = 0;
        
    }
}
            s[i].to_ascii_lowercase();
        }
    }
    let mut ss = String::new();
    for(i, c) in s.chars().enumerate(){
        ss.push(c);
        if i > 0{
            break;
        }
    }
    println!("{}", ss);
    
}


use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use std::io;

#[fastout]
fn main() {
    input!{
        n:usize,
   }
   let mut per = n % 9;
   let mut tmp = n / 9;
   let ans :String = String::new();
   for i in 0..tmp{
       ans.push(per.)
   }

}








use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;
use num::integer::gcd;
use num::integer::sqrt;
use std::boxed::Box;
use std::collections::VecDeque;

fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }
    let mut tmp = a[0];
    let mut counter = 1;
    let mut que = Vec::new();
    let mut x = 0;
    que.push(a[0]);
    println!("1");

    for i in 1..n{
        que.push(a[i]);
        
        if tmp == a[i]{
            counter += 1;
            if counter == a[i]{use proconio::{fastout, input, marker::Chars};
            use std::collections::*;
            use std::collections::HashSet;
            use std::cmp;
            use core::cmp::*;
            use std::io;
            use num::integer::lcm;
            use libm::atan;
            use std::convert::TryInto;
            use num::integer::gcd;
            use num::integer::sqrt;
            use std::boxed::Box;
            use std::collections::VecDeque;
            
            fn main() {
                input! {
                    n:usize,
                    m:usize,
                    a:[usize; n],
                    b:[usize; m],
                }
                let mut aa = HashMap::new();
                let mut bb = HashMap::new();
                for x in a{
                    let counter = aa.entry(x).or_insert(0);
                    *counter += 1;
                }
                for x in b{
                    let counter = bb.entry(x).or_insert(0);
                    *counter += 1;
                }
                for (key, value) in bb{
                    if aa.contains_key(&key){
                        let value2 = aa.get(&key) as usize;
                        aa.insert(key, (value2-1) as String);
                        if aa.get(&key) == 0{
                            aa.remove(&key);
                        }
                    } else {
                        println!("No");
                        return;
                    }
                }
                println!("Yes");use proconio::{fastout, input, marker::Chars};
                use std::collections::*;
                use std::collections::HashSet;
                use std::cmp;
                use core::cmp::*;
                use std::io;
                use num::integer::lcm;
                use libm::atan;
                use std::convert::TryInto;
                use num::integer::gcd;
                use num::integer::sqrt;
                use std::boxed::Box;
                use std::collections::VecDeque;
                
                fn main() {
                    input! {
                        n:usize,
                        m:usize,
                        a:[usize; n],
                        b:[usize; m],
                    }
                    let mut aa = HashMap::new();
                    let mut bb = HashMap::new();
                    for x in a{
                        let counter = aa.entry(x).or_insert(0);
                        *counter += 1;
                    }
                    for x in b{
                        let counter = bb.entry(x).or_insert(0);
                        *counter += 1;
                    }
                    for (key, value) in bb{
                        if aa.contains_key(&key){
                            let value2 = aa.get(&key) as usize;
                            aa.insert(key, (value2-1) as String);
                            if aa.get(&key) == 0{
                                aa.remove(&key);
                            }
                        } else {
                            println!("No");
                            return;
                        }
                    }
                    println!("Yes");
                }
                x = a[i];
            }
        } else {
            counter = 1;
        }
        println!("{}", que.len());
        if que.len() != 0{
            tmp = que[que.len()-1];
        } else {
            tmp = 0;
        }
        x = 0;
        
    }
}

use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;
use num::integer::gcd;
use num::integer::sqrt;
use std::boxed::Box;
use std::collections::VecDeque;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize; n],
        b:[usize; m],
    }
    let mut aa = HashMap::new();
    let mut bb = HashMap::new();
    for x in a{
        let counter = aa.entry(x).or_insert(0);
        *counter += 1;
    }
    for x in b{
        let counter = bb.entry(x).or_insert(0);
        *counter += 1;
    }
    for (key, value) in bb{
        if aa.contains_key(&key){
            let value2 = aa.get(&key) as usize;
            aa.insert(key, (value2-1) as String);
            if aa.get(&key) == 0{
                aa.remove(&key);
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}




use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;
use num::integer::gcd;
use num::integer::sqrt;
use std::boxed::Box;
use std::collections::VecDeque;

fn main() {
    input! {
        mut n:usize,
        m:usize,
        mut a:[usize; n],
        mut b:[usize; m],
    }
    a.sort();
    b.sort();
    let mut len = a.len();
    for i in b{
        let counter = 0;
        for j in a.iter(){
            if j == &i{
                a.remove(counter);
            }
            break;
        }
        len -= 1;
        if a.len() != len {
            println!("No");
            return;
        }
    }
    println!("Yes");
}


use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;
use num::integer::gcd;
use num::integer::sqrt;
use std::boxed::Box;
use std::collections::VecDeque;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 1;
    let mut tmp = 0;
    let mut 1on9 = 0;
    let mut 2on8 = 0;

    for j in 1..=9{
        if j == 1 || j == 9 {

        } else if j == 2 || j == 8 {

        } else if j == 3 || j == 7{

        } else if j == 4 || j == 6 {

        } else if j == 5 {
            
        }
        ans += tmp;
        ans %= 998244353;
    }

    println!("{}", ans);

}