#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}
trait LightTime{
    fn time(&self) -> u8 ;
}

impl LightTime for TrafficLight {
    fn time(&self) -> u8{
        match self {
            TrafficLight::Red => 61,
            TrafficLight::Green => 50,
            TrafficLight::Yellow => 255,
        }

    }
}

fn main() {
    let yellow = TrafficLight::Yellow;
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    println!("yellow(黄色) time is {:?}",yellow.time());
    println!("red（红色） time is {:?}",red.time());
    println!("green（绿色） time is {:?}",green.time());
}
