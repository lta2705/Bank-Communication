use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use chrono::Local;
use tokio::sync::RwLock;

/// STAN (System Trace Audit Number) Generator
/// Generates unique sequential numbers from 000001 to 999999
/// Resets daily at midnight
#[derive(Debug)]
pub struct StanGenerator {
    current: AtomicU32,
    last_date: Arc<RwLock<String>>,
}

impl StanGenerator {
    /// Create a new STAN generator
    pub fn new() -> Self {
        Self {
            current: AtomicU32::new(0),
            last_date: Arc::new(RwLock::new(Self::get_current_date())),
        }
    }

    /// Get next STAN number
    /// Returns a 6-digit string (000001-999999)
    pub async fn next(&self) -> String {
        let current_date = Self::get_current_date();
        
        // Check if date has changed
        let mut last_date = self.last_date.write().await;
        if *last_date != current_date {
            // Reset counter for new day
            self.current.store(0, Ordering::SeqCst);
            *last_date = current_date;
            tracing::info!("STAN counter reset for new day: {}", last_date);
        }
        drop(last_date);

        // Increment and get next value
        let next_val = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        
        // Wrap around if exceeds 999999
        let stan = if next_val > 999999 {
            self.current.store(1, Ordering::SeqCst);
            1
        } else {
            next_val
        };

        format!("{:06}", stan)
    }

    /// Get current STAN value without incrementing
    pub fn current(&self) -> u32 {
        self.current.load(Ordering::SeqCst)
    }

    /// Get current date as YYYYMMDD string
    fn get_current_date() -> String {
        Local::now().format("%Y%m%d").to_string()
    }

    /// Reset counter (for testing or manual reset)
    pub async fn reset(&self) {
        self.current.store(0, Ordering::SeqCst);
        let mut last_date = self.last_date.write().await;
        *last_date = Self::get_current_date();
        tracing::info!("STAN counter manually reset");
    }
}

impl Default for StanGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stan_generation() {
        let generator = StanGenerator::new();
        
        let stan1 = generator.next().await;
        let stan2 = generator.next().await;
        let stan3 = generator.next().await;
        
        assert_eq!(stan1, "000001");
        assert_eq!(stan2, "000002");
        assert_eq!(stan3, "000003");
    }

    #[tokio::test]
    async fn test_stan_format() {
        let generator = StanGenerator::new();
        
        let stan = generator.next().await;
        assert_eq!(stan.len(), 6);
        assert!(stan.chars().all(|c| c.is_digit(10)));
    }

    #[tokio::test]
    async fn test_stan_reset() {
        let generator = StanGenerator::new();
        
        let _ = generator.next().await;
        let _ = generator.next().await;
        
        generator.reset().await;
        
        let stan = generator.next().await;
        assert_eq!(stan, "000001");
    }
}
