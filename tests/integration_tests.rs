use claude_code_rs::{Task, Tool, Context, Cost};

#[tokio::test]
async fn test_01_task_id_generation() {
    let task_type = Task::TaskType::LocalBash;
    let id = Task::generate_task_id(task_type);
    assert!(id.starts_with("b"));
    assert_eq!(id.len(), 9); // 'b' + 8 chars
}

#[tokio::test]
async fn test_02_tool_registry_registration() {
    let registry = Tool::ToolRegistry::new();
    let tools = registry.list_tools();
    assert!(tools.iter().any(|t| t["name"] == "read_file"));
    assert!(tools.iter().any(|t| t["name"] == "write_file"));
    assert!(tools.iter().any(|t| t["name"] == "run_shell_command"));
}

#[tokio::test]
async fn test_03_context_initialization() {
    let ctx = Context::Context::new().await.unwrap();
    assert!(ctx.history.is_empty());
    assert!(ctx.messages.is_empty());
    assert_eq!(ctx.provider, Context::Provider::Google);
}

#[tokio::test]
async fn test_04_cost_tracking() {
    let mut cost = Cost::CostTracker::default();
    cost.add_usage(1000, 1000);
    // 1000 * 0.000015 + 1000 * 0.000075 = 0.015 + 0.075 = 0.09
    assert!(cost.total_cost_usd > 0.089 && cost.total_cost_usd < 0.091);
}

#[test]
fn test_05_circular_buffer() {
    use claude_code_rs::utils::CircularBuffer;
    let mut buffer = CircularBuffer::new(2);
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    let items: Vec<_> = buffer.items().iter().cloned().collect();
    assert_eq!(items, vec![2, 3]);
}

#[test]
fn test_06_string_helpers() {
    use claude_code_rs::utils::{strip_ansi, extract_links, format_size};
    assert_eq!(strip_ansi("\x1b[31mhello\x1b[0m"), "hello");
    assert_eq!(extract_links("Check [Google](https://google.com)"), vec!["https://google.com"]);
    assert_eq!(format_size(1024), "1.00 KB");
}

#[test]
fn test_07_hash_and_uuid() {
    use claude_code_rs::utils::{generate_uuid, hash_string};
    let u1 = generate_uuid();
    let u2 = generate_uuid();
    assert_ne!(u1, u2);
    assert_eq!(hash_string("test"), hash_string("test"));
}

#[tokio::test]
async fn test_08_migration_engine() {
    use claude_code_rs::{Migrations, Context};
    let mut ctx = Context::Context::new().await.unwrap();
    ctx.model = "claude-3-5-sonnet-20240620".to_string();
    
    let engine = Migrations::MigrationEngine::new();
    engine.run_all(&mut ctx).unwrap();
    
    assert_eq!(ctx.model, "claude-3-5-sonnet-20241022");
}

#[test]
fn test_09_bash_utils_security() {
    use claude_code_rs::Bash::BashUtils;
    assert!(BashUtils::is_dangerous_command("rm -rf /"));
    assert!(!BashUtils::is_dangerous_command("ls -la"));
    assert_eq!(BashUtils::quote("hello world"), "'hello world'");
}

#[test]
fn test_10_permissions_engine() {
    use claude_code_rs::{Permissions, Types};
    use std::path::PathBuf;
    
    let base = PathBuf::from("/test");
    let mut engine = Permissions::PermissionsEngine::new(base.clone());
    
    engine.add_rule(base.join("src"), Types::PermissionMode::Auto);
    
    assert_eq!(engine.check_path(&base.join("src/main.rs")), Types::PermissionMode::Auto);
    assert_eq!(engine.check_path(&base.join("other.rs")), Types::PermissionMode::Manual);
    assert!(engine.is_path_safe(&base.join("src")));
    assert!(!engine.is_path_safe(&base.join("../etc")));
}

#[test]
fn test_11_file_edit_logic() {
    use claude_code_rs::FileEdit::FileEditTool;
    use std::fs;
    use std::path::PathBuf;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_edit.txt");
    fs::write(&file_path, "line 1\nline 2\nline 3").unwrap();

    // Test read_lines
    let lines = FileEditTool::read_lines(&file_path, 1, 2).unwrap();
    assert_eq!(lines, "line 1\nline 2");

    // Test apply_edit
    FileEditTool::apply_edit(&file_path, "line 2", "line two").unwrap();
    let new_content = fs::read_to_string(&file_path).unwrap();
    assert!(new_content.contains("line two"));

    fs::remove_file(file_path).unwrap();
}



