fn structure() {
    struct User {
        //姓名
        name: String::new(),
        //性别
        sex: String::new(),
        //年龄
        age: u32,
        //身高
        height: f64,
        //体重
        body_weight: f64,
        //幸运数字
        lucky_number: u32,
        //星座
        constellation: String::new(),
        //爱好
        hobby: [str; 5],
        //各科成绩
        grades: [u32; 3],
    }
    fn newUser(
        user_name: &str,
        user_sex: &str,
        user_age: u32,
        user_height: f64,
        user_body_weight: f64,
        user_lucky_number: u32,
        user_constellation: &str,
        user_hobby: [str; 5],
        user_grades: [u32; 3],
    ) {
        let user_name = User {
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
            constellation: user_constellation,
            //爱好
            hobby: user_hobby,
            //各科成绩
            grades: user_grades,
        };
    }
}