use wpilib::{pwm::PWM, *};
pub use wpilib::pwm::PwmSpeedController;
use crate::input::Input;

  pub struct Drivetrain {
    front_left_motor: pwm::PwmSpeedController,
    front_right_motor: pwm::PwmSpeedController,
    back_left_motor: pwm::PwmSpeedController,
    back_right_motor: pwm::PwmSpeedController,
  
    boost_speed: f32,
  }

  impl Drivetrain {
    pub fn new() -> Self {
      let front_left_motor = PwmSpeedController::new(1);
      let front_right_motor = PwmSpeedController::new(2);
      let back_left_motor = PwmSpeedController::new(3);
      let back_right_motor = PwmSpeedController::new(4);

      let boost_speed = 0.75;

      Drivetrain {
        front_left_motor,
        front_right_motor,
        back_left_motor,
        back_right_motor,

        boost_speed
      }
    }

    /**
     * @brief Control set for a mecanum drivebase
     */
    pub fn mecanum_drive(&mut self, inputs: &Input) {
      let speed = inputs.lefty();
      let rotation = inputs.rightx();
      
      if inputs.b() { // boost
        self.boost_speed = 1.0;
      } else { // standard
        self.boost_speed = 0.75;
      }
  
      // Mecanum stuff * boost 
      let front_left_speed: f32 = (speed + rotation) * self.boost_speed;
      let front_right_speed: f32 = (-speed + rotation) * self.boost_speed;
      let back_left_speed: f32 = (-speed + rotation) * self.boost_speed;
      let back_right_speed: f32 = (speed + rotation) * self.boost_speed;
  
      // Set motor speed
      self.front_left_motor.set(front_left_speed);    // The `set_speed` function accepts a f64, 
      self.front_right_motor.set(front_right_speed);  // but the `stick_axis` function `input` uses only returns an f32
      self.back_left_motor.set(back_left_speed);      // Why? I know it's functionally meaningless, but, I want them to match.
      self.back_right_motor.set(back_right_speed);
    }

    /**
     * @brief Control set for a tank drivebase
     */
    pub fn tank_drive(&mut self, inputs: &Input) {
      let left = inputs.lefty();
      let right = inputs.righty();
      
      if inputs.b() { // boost
        self.boost_speed = 1.0;
      } else { // standard
        self.boost_speed = 0.75;
      }
  
      // Tank stuff * boost 
      let left_speed: f32 = (left) * self.boost_speed;
      let right_speed: f32 = (right) * self.boost_speed;
  
      // Set motor speed
      self.front_left_motor.set(left_speed);
      self.back_left_motor.set(left_speed);
      self.front_right_motor.set(right_speed);
      self.back_right_motor.set(right_speed);
    }
  }
  
  