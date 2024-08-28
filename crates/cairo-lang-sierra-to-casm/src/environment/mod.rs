use cairo_lang_sierra::program::StatementIdx;
use frame_state::{FrameState, FrameStateError};
use thiserror::Error;

use self::frame_state::validate_final_frame_state;
use self::gas_wallet::GasWallet;

pub mod ap_tracking;
pub mod frame_state;
pub mod gas_wallet;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum EnvironmentError {
    #[error("Inconsistent ap tracking.")]
    InconsistentApTracking,
    #[error("Inconsistent frame state.")]
    InconsistentFrameState,
    #[error("Inconsistent gas wallet state.")]
    InconsistentGasWallet,
    #[error("{0}")]
    InvalidFinalFrameState(FrameStateError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApTrackingBase {
    /// The base is a statement that enabled ap tracking.
    Statement(StatementIdx),
    /// The base is the beginning of the function.
    FunctionStart,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApTracking {
    Disabled,
    Enabled {
        /// The ap change between `base` and the current statement.
        ap_change: usize,
        /// The statement that enabled the ap tracking.
        base: ApTrackingBase,
    },
}

/// Part of the program annotations that libfuncs may access as part of their run.
#[derive(Clone, Debug)]
pub struct Environment {
    /// The ap tracking information of the current statement.
    pub ap_tracking: ApTracking,
    /// The size of the continuous known stack.
    pub stack_size: usize,
    pub frame_state: FrameState,
    pub gas_wallet: GasWallet,
}
impl Environment {
    pub fn new(gas_wallet: GasWallet) -> Self {
        Self {
            ap_tracking: ApTracking::Enabled { ap_change: 0, base: ApTrackingBase::FunctionStart },
            stack_size: 0,
            frame_state: FrameState::BeforeAllocation,
            gas_wallet,
        }
    }
}

// Validates that the environments match and returns appropriate error if not.
pub fn validate_environment_equality(
    a: &Environment,
    b: &Environment,
) -> Result<(), EnvironmentError> {
    if a.ap_tracking != b.ap_tracking {
        Err(EnvironmentError::InconsistentApTracking)
    } else if a.frame_state != b.frame_state {
        Err(EnvironmentError::InconsistentFrameState)
    // } else if a.gas_wallet != b.gas_wallet {
    //     Err(EnvironmentError::InconsistentGasWallet)
    } else {
        Ok(())
    }
}

// Validates that the state at the end of a function is valid.
pub fn validate_final_environment(env: &Environment) -> Result<(), EnvironmentError> {
    validate_final_frame_state(&env.frame_state).map_err(EnvironmentError::InvalidFinalFrameState)
}
