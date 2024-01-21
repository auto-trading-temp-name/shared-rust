use chrono::Duration;
use clokwerk::Interval;

pub struct CustomInterval(pub Duration);

impl CustomInterval {
	pub fn interval(&self) -> Interval {
		Interval::Seconds(
			self
				.0
				.num_seconds()
				.try_into()
				.expect("interval should be positive"),
		)
	}

	pub fn duration(&self) -> Duration {
		self.0
	}

	pub fn std_duration(&self) -> std::time::Duration {
		self.0.to_std().expect("interval should be positive")
	}
}
