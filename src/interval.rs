use chrono::TimeDelta;
use clokwerk::Interval;
use eyre::Result;
use std::time::Duration;

pub struct CustomInterval(pub Duration);

impl CustomInterval {
	pub fn interval(&self) -> Interval {
		Interval::Seconds(self.0.as_secs() as u32)
	}

	pub fn duration(&self) -> Result<chrono::TimeDelta> {
		Ok(TimeDelta::from_std(self.0)?)
	}

	pub fn std_duration(&self) -> Duration {
		self.0
	}
}
