use wpilib::ds::{*};

pub trait Input {
  // Functions we need to implement for a given input mechanism.

  fn a(&self) -> bool; 

  fn b(&self) -> bool; 
  
  fn x(&self) -> bool;

  fn y(&self) -> bool;

  fn lb(&self) -> bool; 

  fn rb(&self) -> bool; 
  
  fn leftx(&self) -> f32;
  
  fn lefty(&self) -> f32;

  fn rightx(&self) -> f32;
  
  fn righty(&self) -> f32;

  const A: u8 = 1;
  const B: u8 = 2;
  const X: u8 = 3;
  const Y: u8 = 4;

  const LB: u8 = 5;
  const RB: u8 = 6;

  const BACK: u8 = 7;
  const START: u8 = 8;
	
  const LEFT_X: u8 = 0; // To-Do: Check that these are the correct values for said stick.
  const LEFT_Y: u8 = 1;
  const RIGHT_X: u8 = 2;
  const RIGHT_Y: u8 = 3;

}

pub struct XboxController<'a> {
  ds: &'a DriverStation<'a>,
  port: JoystickPort
}

impl XboxController<'_> {
  fn raw_axis(&self, axis: u8) -> f32 {
      self.ds
          .stick_axis(self.port, JoystickAxis::new(axis).unwrap())
          .unwrap_or(0.0)
  }
	
  fn raw_button(&self, button: u8) -> bool {
      self.ds.stick_button(self.port, button).unwrap_or(false)
  }
}


impl Input for XboxController<'_> {

  fn a(&self) -> bool {
    self.raw_button(Self::A)
  }

  fn b(&self) -> bool {
    self.raw_button(Self::B)
  }

  fn x(&self) -> bool {
    self.raw_button(Self::X)
  }

  fn y(&self) -> bool {
    self.raw_button(Self::Y)
  }

  fn lb(&self) -> bool {
    self.raw_button(Self::LB)
  }

  fn rb(&self) -> bool {
    self.raw_button(Self::RB)
  }

  fn leftx(&self) -> f32 {
    self.raw_axis(Self::LEFT_X)
  }

  fn lefty(&self) -> f32 {
    self.raw_axis(Self::LEFT_Y) 
  }

  fn rightx(&self) -> f32 {
    self.raw_axis(Self::RIGHT_X)
  }
	
  fn righty(&self) -> f32 {
    self.raw_axis(Self::RIGHT_Y)
  }
}
