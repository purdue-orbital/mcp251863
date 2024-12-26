use bitbybit::{bitenum, bitfield};
use arbitrary_int::u3;

// note: see CiCON register
#[derive(Debug)]
#[bitenum(u3, exhaustive = true)]
pub enum OperationMode {
	/// supports mixing of CAN 2.0 and FD
	Normal = 0,
	/// CAN FD frames can become error frames
	NormalCAN2 = 6,
	Config = 4,
	ListenOnly = 3,
	Sleep = 1,
	InternalLoopback = 2,
	ExternalLoopback = 5,
	RestrictedOperation = 7,
}