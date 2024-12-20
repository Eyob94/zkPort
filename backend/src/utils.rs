use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    pub max_retries: u64,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    // Use 1 if no exponential_factor
    pub exponential_factor: u32,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 3000,
            max_delay_ms: 3000,
            exponential_factor: 1,
        }
    }
}

pub async fn execute_with_retry<F, T>(
    mut operation: F,
    retry_policy: RetryPolicy,
) -> anyhow::Result<T>
where
    F: FnMut() -> anyhow::Result<T>,
{
    let mut retries = 0;
    let mut delay = retry_policy.base_delay_ms;

    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) if retries < retry_policy.max_retries => {
                retries += 1;
                let jitter = rand::random::<u64>() % 1000; // Add up to 1 second of jitter
                let total_delay = std::cmp::min(
                    delay * retry_policy.exponential_factor as u64 + jitter,
                    retry_policy.max_delay_ms,
                );
                sleep(Duration::from_millis(total_delay)).await;
                delay = total_delay;
            }
            Err(e) => return Err(e),
        }
    }
}
