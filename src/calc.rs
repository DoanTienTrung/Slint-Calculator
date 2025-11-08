// mo ta trang thai
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
  Add,
  Sub,
  Mul,
  Div,
}


pub struct CalculatorState {
  pub current: String,
  pub stored: f64,
  pub op: Option<Op>,
  pub last_operand: Option<f64>,
  pub error: bool,

}
// ham khoi tao - khi mo app CalculatorState::new() se tra ve trang thai ban dau
impl CalculatorState {
  pub fn new() -> Self {
    Self {
      current: "0".to_string(),
      stored: 0.0,
      op: None,
      last_operand: None,
      error: false,
    }
  }
  // nhap so input
  pub fn input_digital(&mut self, d: char) {  //&mut self cho phep sua self
     if self.error {
      *self = CalculatorState::new(); // neu dang loi, reset ve trang thai ban dau truoc khi nhap so moi
     }
     if self.current == "0" {
      self.current = d.to_string();
     } else {
      self.current.push(d);
     }
  }

  // xu ly phep toan
  pub fn set_op(&mut self, op: Op) {
    if self.error {
      return;
    }
    // neu nguoi dung vua nhap so, luu lai so do vao stored
    if let Ok(value) = self.current.parse::<f64>() {  // parse f64 chuyen chuoi so "12.3" sang so thuc 12.3
      self.stored = value;  // neu ma chuyen doi so Ok thi luu vao stored
      self.current = "0".to_string(); // reser current de nhap so tiep theo
      self.op = Some(op);
    }
  }

  // ham evaluate
    pub fn evaluate(&mut self) {
      if self.error {
        return;
      }
      
      if let (Some(op), Ok(rhs)) = (self.op, self.current.parse::<f64>()) {
        let lhs = self.stored;
        let result = match op {
          Op::Add => lhs + rhs,
          Op::Sub => lhs - rhs,
          Op::Mul => lhs * rhs,
          Op::Div => {
            if rhs == 0.0 {
              self.error = true;
              f64::NAN
            } else {
              lhs / rhs
            }
          }
        };

        if !self.error {
          self.current = result.to_string(); // hien thi ket qua ra man hinh
          self.stored = result; // luu ket qua de thuc hien phep tinh tiep theo
          self.last_operand = Some(rhs); // lưu giá trị rhs để có thể lặp phép tính
        } else {
          self.current = "Error".to_string();
        }
      }
    }

    // ham backspace
    pub fn backspace(&mut self) {
      if self.error {
        return;
      }
      if self.current.len() > 1 {
        self.current.pop(); // xóa ký tự cuối
      } else {
        self.current = "0".to_string();
      }
    }

    // ham doi dau phep tinh
    pub fn toggle_sign(&mut self){
      if self.error {
        return;
      }
      if self.current.starts_with('-') {
      // neu dang co dau am, bo di
      self.current.remove(0);
    } else if self.current != "0" {
      // neu khong phai ), them dau am
      self.current.insert(0, '-');
    }
    }

    // ham xu ly so thap phan
    pub fn input_dot(&mut self) {
      if self.error {
        *self = CalculatorState::new();
      }
      // neu chua co dau . thi them vao
      if !self.current.contains('.') {
        self.current.push('.');
      }
    }

    // ham xu ly tinh phan tram
    pub fn percent(&mut self) {
      if self.error {
        return;
      }
      if let Ok(value) = self.current.parse::<f64>() { // chuyen sang so thuc
        let result = value / 100.0;
        self.current = result.to_string();
      }
    }
    
  

}

// test nhap so
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_digital() {
        let mut calc = CalculatorState::new();
        calc.input_digital('1');
        calc.input_digital('2');
        assert_eq!(calc.current, "12");
    }
}


#[test]
fn test_addition() {
  let mut calc = CalculatorState::new();
  calc.input_digital('1');
  calc.input_digital('2'); //12
  calc.set_op(Op::Add);
  calc.input_digital('3');
  calc.evaluate();
  assert_eq!(calc.current, "15");


}