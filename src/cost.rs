use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct CostTracker {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_cost_usd: f64,
}

impl CostTracker {
    pub fn add_usage(&mut self, input: u64, output: u64) {
        self.input_tokens += input;
        self.output_tokens += output;
        // Approximation: $15/1M input, $75/1M output
        self.total_cost_usd += (input as f64 * 0.000015) + (output as f64 * 0.000075);
    }

    pub fn display_string(&self) -> String {
        format!(
            "Cost: ${:.4} | Tokens: {} IN / {} OUT",
            self.total_cost_usd, self.input_tokens, self.output_tokens
        )
    }
}
