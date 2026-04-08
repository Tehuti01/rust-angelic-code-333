use regex::Regex;

pub struct BashUtils;

impl BashUtils {
    // Ported from utils/bash/shellQuote.ts
    pub fn quote(s: &str) -> String {
        if s.is_empty() {
            return "''".to_string();
        }
        if !Regex::new(r"[^A-Za-z0-9_\-.,:/@\n]").unwrap().is_match(s) {
            return s.to_string();
        }
        format!("'{}'", s.replace("'", "'\\''"))
    }

    // Ported from utils/bash/bashSecurity.ts
    pub fn is_dangerous_command(command: &str) -> bool {
        let dangerous_patterns = [
            r"rm\s+-rf\s+/",
            r":\(\)\{.*\}", // Fork bomb
            r"mv\s+.*\s+/dev/null",
            r"dd\s+if=/dev/zero",
        ];
        
        for pattern in dangerous_patterns {
            if Regex::new(pattern).unwrap().is_match(command) {
                return true;
            }
        }
        false
    }

    // Ported from utils/bash/parser.ts (simplified)
    pub fn parse_environment_variables(command: &str) -> std::collections::HashMap<String, String> {
        let mut envs = std::collections::HashMap::new();
        let re = Regex::new(r"([A-Z_]+)=([^\s]+)").unwrap();
        for cap in re.captures_iter(command) {
            envs.insert(cap[1].to_string(), cap[2].to_string());
        }
        envs
    }
}
