use events::Event::*;
use chrono::Duration;

fn main() {
	let remainder = Remainder("Go to the doctor");
	println!("{}", remainder.notify());
	let registration = Registration(Duration::seconds(49094));
	println!("{}", registration.notify());
	let appointment = Appointment("Go to the doctor");
	println!("{}", appointment.notify());
	let holiday = Holiday;
	println!("{}", holiday.notify());
}
/*

(Bottom, 50, [38;2;50;50;50mGo to the doctor[0m)
(Top, 30, [38;2;255;2;22mYou have 13H:38M:14S left before the registration ends[0m)
(Center, 100, [38;2;200;200;3mGo to the doctor[0m)
(Top, 25, [38;2;0;255;0mEnjoy your holiday[0m)

*/