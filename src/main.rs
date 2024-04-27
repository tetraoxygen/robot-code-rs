// crates
use wpilib::{ds::{Alliance, JoystickPort}, *};
use drivetrain::Drivetrain;
use input::{Input, XboxController};

// our files
mod input;
mod drivetrain;

fn main() {
    // MARK: Setup
    RobotBase::start_competition();
    let robot = RobotBase::new().expect("HAL FAILED");
    let mut drivebase = Drivetrain::new();
    let ds = robot.make_ds();

    // To-Do: Assign DS & Joystick port 
    // Joystick port can be created using ds::JoystickPort::new(u8)
    let driverController = XboxController{ ds: todo!(), port: todo!() };

    // MARK: Match code

    // Some sample boilerplate:
    let alliance = ds.alliance().unwrap();
    match alliance {
        Alliance::Red => {
            println!("Red Alliance")
        }
        Alliance::Blue => {
            println!("Blue Alliance")
        }
    }

    while ds.robot_state() == wpilib::ds::RobotState::Teleop {
        drivebase.tank_drive(&driverController);
    }
    
}

