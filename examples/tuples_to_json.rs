use serde::{Deserialize, Serialize};



fn tuples() -> Vec<(i32, &'static str, &'static str, i32, &'static str, i32, i32, &'static str)> {
    let x = vec![
        (1,"2017-01-12 19:34:08","Butt Kickers",1,"Both",30,1,"{name}. {side} side."),
(2,"2017-01-12 19:34:08","Clamshells",1,"Both",30,2,"10 {name}. {side} side. Keep the knee in line"),
(3,"2017-01-12 19:34:08","High Kicks",2,"Null",50,1,"{name}"),
(4,"2017-01-12 19:34:08","Curls",3,"Null",60,3,"{name}"),
(5,"2017-01-12 19:34:08","Calf Stretch",2,"Both",40,1,"{name}. {side} side."),
(6,"2017-01-12 19:34:08","Calf Lifts",3,"Null",30,3,"10 {name}"),
(7,"2017-01-12 19:34:08","Bird bops",1,"Both",50,1,"{name}. {side} side."),
(8,"2017-01-12 19:34:08","Normal Pushups",3,"Null",50,1,"{name}"),
(9,"2017-01-12 19:34:08","Butterfly Stretch",2,"Null",50,1,"{name}"),
(10,"2017-01-12 19:34:08","Groin flex",1,"Null",50,1,"{name}. 10 reps of 5 seconds"),
(11,"2017-01-12 19:34:08","Bridge",1,"Null",20,1,"{name}. 10 reps"),
(12,"2017-01-12 19:34:08","One foot bridge",1,"Both",15,1,"{name}. {side} side. 10 reps"),
(13,"2017-01-12 19:34:08","Diagonal backward fondas",1,"Both",25,2,"{name}. {side} side. 10 reps"),
(14,"2017-01-12 19:34:08","Situps",3,"Null",80,1,"{name}. 50 reps"),
(15,"2017-01-12 19:34:08","Incline Pushups",3,"Null",50,1,"{name}"),
(16,"2017-01-12 19:34:08","Band Walk",1,"Null",80,1,"{name}"),
(17,"2017-01-12 19:34:08","Straight Plank",3,"Null",55,1,"{name}. 50 seconds"),
(18,"2017-01-12 19:34:08","Side Plank",3,"Both",55,1,"{name}. {side} side. 50 seconds"),
(19,"2017-01-12 19:34:08","Leg circles",1,"Both",10,4,"{name}. {side} side. Switch direction every other one"),
(20,"2017-01-12 19:34:08","Roller on the back",4,"Null",80,1,"{name}"),
(21,"2017-01-12 19:34:08","Legs crossed stretch",2,"Both",50,1,"{name}. {side} side."),
(22,"2017-01-12 19:34:08","Ninja stretch",2,"Both",50,1,"{name}. {side} side."),
(23,"2017-01-12 19:34:08","Squats",3,"Null",40,2,"15 {name}"),
(24,"2017-01-12 19:34:08","Quad Stretch",2,"Both",40,1,"{name}. {side} side"),
(25,"2017-01-12 19:34:08","Narrow Pushups",3,"Null",50,1,"{name}"),
(26,"2017-01-12 19:34:08","Skaters",1,"Both",30,1,"{name}. {side} side."),
(27,"2017-01-12 19:34:08","Pullups",3,"Null",60,1,"{name}"),
(28,"2017-01-12 19:34:08","Roller on the groin",4,"Both",55,1,"{name}. {side} side."),
(29,"2017-01-12 19:34:08","Lacrosse Ball on the groin",4,"Both",55,1,"{name}. {side} side."),
(30,"2017-01-16 00:00:00","Half Roller Squats",3,"Both",35,2,"{name}. {side} side"),
(31,"2017-01-16 00:00:00","Back curls",3,"Null",30,3,"{name}"),
(34,"2017-01-17 21:05:28","Roller on the glutes",4,"Both",60,1,"{name}. {side} side"),
(35,"2017-01-26 22:00:00","Warrior 2",2,"Both",30,2,"{name}. {side} side"),
(36,"2017-01-31 00:00:00","Hamstring stretch",2,"Both",40,1,"{name}. {side} side."),
(37,"2017-02-12 00:00:00","Knee to chest stretch",2,"Both",40,1,"{name}. {side} side."),
(38,"2017-03-06 00:00:00","Squat hold",3,"Null",40,2,"{name}. Take 10 seconds off in between."),
(39,"2017-03-06 00:00:00","Dips",3,"Null",30,2,"{name}. 10 reps"),
(40,"2017-03-06 00:00:00","Bicycle crunches",3,"Null",80,1,"{name}"),
(41,"2017-03-06 00:00:00","Mountain climbers",3,"Null",40,2,"10 {name} each side."),
(42,"2017-03-06 00:00:00","Lunges",3,"Null",60,1,"{name}"),
(43,"2017-03-14 00:00:00","Hand roller on the groin",4,"Both",55,1,"{name}. {side} side."),
(44,"2018-10-11 00:00:00","Inner thigh roll",4,"Both",60,1,"{name}. {side} side."),
(45,"2018-10-11 00:00:00","Lacrosse ball on hip",4,"Both",60,1,"{name}. {side} side."),
(46,"2018-10-11 00:00:00","Hamstring Roller",4,"Both",60,1,"{name}. {side} side."),
(47,"2018-10-11 00:00:00","Lacrosse ball on upper knee",4,"Both",60,1,"{name}. {side} side."),
(48,"2018-10-11 00:00:00","Lacrosse ball behind knee",4,"Both",50,1,"{name}. {side} side."),
(49,"2018-10-11 00:00:00","Roller on the quad",4,"Both",120,1,"{name}. {side} side."),
(50,"2018-10-11 00:00:00","Lacrosse ball on glutes",4,"Both",60,1,"{name}. {side} side."),
(51,"2018-12-20 00:00:00","Calf roller",4,"Both",50,1,"{name}. {side} side"),
(52,"2019-12-27 00:00:00","Groin Squeeze",1,"Null",65,3,"Groin squeeze. 60 seconds. Get wider between each rep"),
(53,"2019-12-27 00:00:00","Hip mobility",2,"Both",60,2,"{name}. Bend {side} side in. "),
(54,"2019-12-27 00:00:00","Glute raises",1,"Both",100,2,"{name}. {side} side. 10 reps"),
(55,"2020-01-01 00:00:00","Glute stretch",2,"Both",60,1,"Plank {name}. {side} side. "),
(56,"2020-01-01 00:00:00","1 legged squats",3,"Both",50,3,"{name}. {side} side. 10 reps"),
(57,"2019-01-06 00:00:00","Ottoman step ups",2,"Both",60,3,"{name}. {side} side. 10 reps")
    ];
    x
}

#[derive(Serialize, Deserialize)]
struct JsonOutput {
    date_added: String,
    name: String,
    category_id: u8,
    side: String,
    default_time: u8,
    repetition: u8,
    prompt: String
}

fn main() {
    // Write a method to convert the tuples to json
    let x = tuples();
    let mut y: Vec<JsonOutput> = Vec::new();
    for i in x {
        let z = JsonOutput {
            date_added: i.1.to_string(),
            name: i.2.to_string(),
            category_id: i.3 as u8,
            side: i.4.to_string(),
            default_time: i.5 as u8,
            repetition: i.6 as u8,
            prompt: i.7.to_string()
        };
        y.push(z);
    }
    let json = serde_json::to_string_pretty(&y).unwrap();
    //Write the json to a file
    std::fs::write("exercises.json", json).expect("Unable to write file");
    // println!("{}", json);
}