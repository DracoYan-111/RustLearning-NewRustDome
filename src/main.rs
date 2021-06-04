use std::io;
use rand::Rng;
use std::cmp::Ordering;
use chrono::prelude::*;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::ParseResult;


fn main() {
    //键盘输入用户名称
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
    }
    //键盘输入用户出生年月
    println!("请输入出生年");
    let time: u32 = GrtTime();
    let mut UserAge: u32;
    let mut UserAgeTwo: String = String::new();
    loop {
        io::stdin().read_line(&mut UserAgeTwo).expect("接收失败");
        let UserAgeTwo: u32 = match UserAgeTwo.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("请重新输入");
                continue;
            }
        };
        if UserAgeTwo <= time && UserAgeTwo > 1940 {
            UserAge = time - UserAgeTwo;
            break;
        } else {
            println!("请重新输入");
            continue;
        }
    }
    println!("请输入用户身高");
    let mut UserHeight: f64;
    loop {
        let mut UserHeightTwo: String = String::new();
        io::stdin().read_line(&mut UserHeightTwo).expect("接收失败");
        let UserHeightTwo: f64 = match UserHeightTwo.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("请重新输入");
                continue;
            }
        };
        if UserHeightTwo < 280.0 && UserHeightTwo > 50.0 {
            UserHeight = UserHeightTwo;
            break;
        } else {
            println!("请重新输入");
            continue;
        }
    }
    println!("请输入用户体重");
    let mut UserBodyWeight: f64;
    loop {
        let mut UserBodyWeightTwo: String = String::new();
        io::stdin().read_line(&mut UserBodyWeightTwo).expect("接收失败");
        let UserBodyWeightTwo: f64 = match UserBodyWeightTwo.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("请重新输入");
                continue;
            }
        };
        if UserBodyWeightTwo < 500.0 && UserBodyWeightTwo > 15.0 {
            UserBodyWeight = UserBodyWeightTwo;
            break;
        } else {
            println!("请重新输入");
            continue;
        }
    }

    let Constellation: [&str; 12] = ["白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座", "天秤座", "天蝎座", "射手座", "摩羯座", "水瓶座", "双鱼座"];
    println!("请输入用户星座");
    let mut UserConstellation: String = String::new();
    loop {
        let mut Pan: bool = false;
        let mut UserConstellationTwo: String = String::new();
        io::stdin().read_line(&mut UserConstellationTwo).expect("接收失败");
        for i in 0..Constellation.len()
        {
            if (UserConstellationTwo.trim()).eq(Constellation[i]) {
                UserConstellation = UserConstellationTwo;
                Pan = true;
                break;
            } else {
                continue;
            }
        }
        if Pan == true {
            break;
        } else {
            println!("请重新输入");
            continue;
        }
    }

    let mut UserLuckyNumber: u32 = rand::thread_rng().gen_range(1, 101);
    println!("用户姓名为{}", UserName);
    println!("用户性别为:{}", UserSex);
    println!("用户年龄为:{}", UserAge);
    println!("用户身高为:{}", UserHeight);
    println!("用户体重为:{}", UserBodyWeight);
    println!("用户幸运数字为:{}", UserLuckyNumber);
    println!("用户星座为:{}", UserConstellation);

    structure(UserName,
              UserSex,
              UserAge,
              UserHeight,
              UserBodyWeight,
              UserLuckyNumber,
              UserConstellation);
}


fn GrtTime() -> u32 {
    //let fmt = "%Y-%m-%d %H:%M:%S" 完整写发
    //时间转型
    let fmt = "%Y";
    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string(); // 2021-01-04 20:02:09
    let str_dates: u32 = str_date.trim().parse().expect("错误");
    str_dates
}

fn structure(
    user_name: String,
    user_sex: String,
    user_age: u32,
    user_height: f64,
    user_body_weight: f64,
    user_lucky_number: u32,
    user_constellation: String,
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
}