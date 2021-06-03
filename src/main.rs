use std::io;
use rand::Rng;
use std::cmp::Ordering;
use chrono::prelude::*;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::ParseResult;


fn main() {
    /* //键盘输入用户名称
     println!("请输入名称");
     let mut UserName: String = String::new();
     io::stdin().read_line(&mut UserName).expect("接收失败");
     //键盘输入用户性别
     println!("请输入性别 1:男 2:女");
     let mut UserSex: String = String::new();
     loop {
         let mut UserSexTwo: String = String::new();
         io::stdin().read_line(&mut UserSexTwo).expect("接收失败");
         let UserSexTwo: u32 = match UserSexTwo.trim().parse()
         {
             Ok(num) => num,
             Err(_) => {
                 println!("请重新输入");
                 continue;
             }
         };
         if UserSexTwo == 1 {
             UserSex = "男".parse().unwrap();
             break;
         }
         if UserSexTwo == 2
         {
             UserSex = "女".parse().unwrap();
             break;
         } else {
             println!("请重新输入!");
             continue;
         }
     }*/
    //键盘输入用户出生年月
    println!("请输入出生年");
    //let fmt = "%Y-%m-%d %H:%M:%S" 完整写发
    //时间转型
    let time: u32 = GrtTime();
    loop {
        let mut UserAge: String = String::new();
        io::stdin().read_line(&mut UserAge).expect("接收失败");
        let UserAge: u32 = match UserAge.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("请重新输入");
                continue;
            }
        };
    }

    println!("now: {}", str_dates);

    /*println!("用户姓名{}", UserName);
    println!("用户性别{}", UserSex);*/

    /*/* fn test() -> i32 {
         let a = [2, 2, 6, 4, 5];
         let mut count: i32 = 0;
         for i in 0..5 {
             count = count + a[i];
         }
         count
     }


     println!("游戏开始");
     loop {
         println!("请猜一个数字");
         /*let mut fee = String::new();
         io::stdin().read_line(&mut fee).expect("接收失败");
         let fee: u32 = match fee.trim().parse()
         {
             Ok(num) => num,
             Err(_) => continue,
         };*/

         let ser: i32 = rand::thread_rng().gen_range(1, 101);

         if test() < ser {
             println!("太大了");
         }
         if test() == ser {
             println!("答对了");
             break;
         }
         if test() > ser {
             println!("太小了");
         }
         println!("------{}", ser);
     }
     println!("//////{}", test());

     */

    struct User {
        name: String,
        age: u32,
        sex: String,
        grades: [i32;3],
    }

    let vvv:usize = rand::thread_rng().gen_range(0, 3);
    let degree: [&str; 3] = ["简单","中等","复杂"];
    let user_test = User {
        name: String::from("RUST"),
        age: rand::thread_rng().gen_range(1, 101),

        sex: String::from(degree[vvv]),
        grades:[rand::thread_rng().gen_range(1, 101),rand::thread_rng().gen_range(1, 101),rand::thread_rng().gen_range(1, 101)],
    };
    println!("user name == {} ", user_test.name);
    println!("user age == {} ", user_test.age);
    println!("user sex == {} ", user_test.sex);

    for i  in 0..3 {
        println!("user grades == {} ",user_test.grades[i]);
    }*/
}

fn GrtTime() -> u32 {
    let fmt = "%Y";
    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string(); // 2021-01-04 20:02:09
    let str_dates: u32 = str_date.trim().parse().expect("错误");
    str_dates
}

fn structure(
    user_name: &str,
    user_sex: &str,
    user_age: u32,
    user_height: f64,
    user_body_weight: f64,
    user_lucky_number: u32,
    user_constellation: &str,
    /*    user_hobby: [str; 5],
        user_grades: [u32; 3],*/
) {
    struct User {
        //姓名
        name: String,
        //性别
        sex: String,
        //年龄
        age: u32,
        //身高
        height: f64,
        //体重
        body_weight: f64,
        //幸运数字
        lucky_number: u32,
        //星座
        constellation: String,
        /*        //爱好
                hobby: [str; 5],
                //各科成绩
                grades: [u32; 3],*/
    }

    let NewUser = User {
        //姓名
        name: String::from(user_name),
        //性别
        sex: String::from(user_sex),
        //年龄
        age: user_age,
        //身高
        height: user_height,
        //体重
        body_weight: user_body_weight,
        //幸运数字
        lucky_number: user_lucky_number,
        //星座
        constellation: String::from(user_constellation).parse().unwrap(),
        /*        //爱好
                hobby: user_hobby,
                //各科成绩
                grades: user_grades,*/
    };

    println!();
}