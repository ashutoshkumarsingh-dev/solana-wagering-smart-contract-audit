use anchor_lang::prelude::*;
use crate::errors::WagerError;

/// Input validation utilities for security
pub mod validation {
    use super::*;

    /// Validates session ID format and length
    pub fn validate_session_id(session_id: &str) -> Result<()> {
        require!(!session_id.is_empty(), WagerError::InvalidSessionId);
        require!(session_id.len() <= 32, WagerError::SessionIdTooLong);
        require!(
            session_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
            WagerError::InvalidSessionIdFormat
        );
        Ok(())
    }

    /// Validates team number (must be 0 or 1)
    pub fn validate_team_number(team: u8) -> Result<()> {
        require!(team == 0 || team == 1, WagerError::InvalidTeamSelection);
        Ok(())
    }

    /// Validates bet amount is within reasonable bounds
    pub fn validate_bet_amount(amount: u64) -> Result<()> {
        require!(amount > 0, WagerError::InvalidBetAmount);
        require!(amount <= 1_000_000_000_000, WagerError::InvalidBetAmount); // Max 1000 tokens
        Ok(())
    }

    /// Validates player address is not default
    pub fn validate_player_address(player: &Pubkey) -> Result<()> {
        require!(*player != Pubkey::default(), WagerError::InvalidPlayer);
        Ok(())
    }

    /// Validates remaining accounts count is within limits
    pub fn validate_remaining_accounts_count(count: usize, max_count: usize) -> Result<()> {
        require!(count <= max_count, WagerError::TooManyRemainingAccounts);
        Ok(())
    }

    /// Validates kill data is legitimate
    pub fn validate_kill_data(killer: &Pubkey, victim: &Pubkey, killer_team: u8, victim_team: u8) -> Result<()> {
        require!(killer != victim, WagerError::InvalidKill);
        require!(killer_team != victim_team, WagerError::InvalidKill);
        validate_player_address(killer)?;
        validate_player_address(victim)?;
        Ok(())
    }
}

/// Safe arithmetic utilities to prevent overflow/underflow
pub mod safe_math {
    use anchor_lang::prelude::*;
    use crate::errors::WagerError;

    /// Safe multiplication with overflow check
    pub fn safe_multiply(a: u64, b: u64) -> Result<u64> {
        a.checked_mul(b).ok_or(error!(WagerError::ArithmeticOverflow))
    }

    /// Safe division with underflow check
    pub fn safe_divide(a: u64, b: u64) -> Result<u64> {
        require!(b > 0, WagerError::ArithmeticError);
        a.checked_div(b).ok_or(error!(WagerError::ArithmeticOverflow))
    }

    /// Safe addition with overflow check
    pub fn safe_add(a: u64, b: u64) -> Result<u64> {
        a.checked_add(b).ok_or(error!(WagerError::ArithmeticOverflow))
    }

    /// Safe subtraction with underflow check
    pub fn safe_subtract(a: u64, b: u64) -> Result<u64> {
        a.checked_sub(b).ok_or(error!(WagerError::ArithmeticUnderflow))
    }

    /// Safe calculation for earnings in pay-to-spawn mode
    pub fn safe_earnings_calculation(kills_and_spawns: u16, session_bet: u64) -> Result<u64> {
        let kills_spawns_u64 = kills_and_spawns as u64;
        let multiplied = safe_multiply(kills_spawns_u64, session_bet)?;
        safe_divide(multiplied, 10)
    }
}

/// Macro for reentrancy guard
#[macro_export]
macro_rules! reentrancy_guard {
    ($game_session:expr) => {
        require!(!$game_session.is_processing, WagerError::AlreadyProcessing);
        $game_session.is_processing = true;
    };
}

/// Macro to release reentrancy guard
#[macro_export]
macro_rules! release_reentrancy_guard {
    ($game_session:expr) => {
        $game_session.is_processing = false;
    };
}