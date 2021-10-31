# oneblockplus-home-course4

## 作业 1
为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同。
### 代码：
```
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
```
### 截图：
<img width="1042" alt="111" src="https://user-images.githubusercontent.com/3039255/139592736-9616f531-425e-43c1-a5ee-1b70da2aa200.png">

## 作业2
实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None。
### 代码
```

pub fn sum_arry(arry: &[u32]) -> Option<u32> {

    let mut sum = 0u32;
    let mut overflow = false;
    for value in arry {
        match (sum.checked_add(*value)) {
            Some(v) => {
                sum = v;
            },
            None => {
                overflow = true;
                break;
            },
        }
    }

    if overflow == true {
        None
    } else {
        Some(sum)
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let arry1 = [100,200,300,300];
        let arry2 = [100,200,u32::MAX,1];
        assert_eq!(sum_arry(&arry1),Some(900));
        assert_eq!(sum_arry(&arry2),None);
    }
}
```


### 截图
<img width="1023" alt="222" src="https://user-images.githubusercontent.com/3039255/139592882-82bcf291-ddf1-4224-a041-5525b723da79.png">

## 作业3
实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。

### 代码
实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。
```

pub trait Area {
    fn area(&self) -> f64;
}

pub fn get_erea<T:Area>(shap: &T) -> f64 {
    shap.area()
}

struct Circle {
    radius: f64,
}

const PI: f64 = 3.14159;
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Triangle {
    bottom_edge: f64, //底边长
    height: f64, //高
}

impl Triangle {
    pub fn new(e: f64, h: f64) -> Self {
        Triangle {
            bottom_edge:e,
            height:h,
        }
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        (self.height * self.bottom_edge) / 2.0
    }
}

struct Square {
    edge_line: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.edge_line * self.edge_line
    }
}

#[cfg(test)]
mod tests {
    use crate::{Circle, get_erea, PI, Square, Triangle};

    #[test]
    fn it_works() {
        let circle = Circle{
            radius: 2.0f64,
        };
        let triangle = Triangle::new(2.0,2.0);
        let square = Square{
            edge_line:2.0,
        };

        assert_eq!(get_erea(&circle), PI*2.0*2.0);
        assert_eq!(get_erea(&triangle), 2.0);
        assert_eq!(get_erea(&square), 4.0);

    }
}
```

### 截图
<img width="1031" alt="333" src="https://user-images.githubusercontent.com/3039255/139592981-3b0f5ed0-866b-48bd-b11b-f3e02e2c1c24.png">


```
