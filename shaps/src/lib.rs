
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
