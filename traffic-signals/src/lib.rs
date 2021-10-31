
pub trait  LightDuration {
    fn light_durations(&self) -> usize{
        return 0;
    }
}

#[derive(Debug)]
pub enum TrafficLight {
    GreenLight,
    YellowLight,
    RedLight,
}


impl LightDuration for TrafficLight {
    fn light_durations(&self) -> usize {
        match self {
            Self::GreenLight => 20,
            Self::YellowLight => 25,
            Self::RedLight => 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let green_light = TrafficLight::GreenLight; //20
        let yellow_light = TrafficLight::YellowLight;//25
        let red_light = TrafficLight::RedLight;// 30

        assert_eq!(green_light.light_durations(),20);
        assert_eq!(yellow_light.light_durations(),25);
        assert_eq!(red_light.light_durations(),30);
    }
}
