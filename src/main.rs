
enum TrafficLight{
    Red,
    Green,
    Yellow,
}

pub trait GetTrafficLight{
    fn time(&self)->u8;
}
//trait 是一个接口

//定义YELLOW结构体
pub struct YellowLight {
    pub LightTime : u8
}

//定义Red结构体
pub struct RedLight {
    pub LightTime : u8
}

//定义Green结构体
pub struct GreenLight {
    pub LightTime : u8
}

//实现trait
impl GetTrafficLight for YellowLight {
    fn time(&self) -> u8 {
        30
    }
}

//实现trait
impl GetTrafficLight for RedLight {
    fn time(&self) -> u8 {
        20
    }
}

//实现trait
impl GetTrafficLight for GreenLight {
    fn time(&self) -> u8 {
        10
    }
}

fn main() {
    let yellow = YellowLight { LightTime: 0 };
    let red = RedLight { LightTime: 0 };
    let green = GreenLight { LightTime: 0 };
    println!("yellow: {}, red: {}, green {}", yellow.time(), red.time(), green.time());
}
