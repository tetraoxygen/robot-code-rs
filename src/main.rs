// crates
use wpilib::{ds::Alliance, *};

// our files
mod input;
mod drivetrain;

fn main() {
    // MARK: Setup
    RobotBase::start_competition();
    let robot = RobotBase::new().expect("HAL FAILED");
    let ds = robot.make_ds();

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
}