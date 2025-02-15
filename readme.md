# Rusty Task Automator

## What's This?
This is **Rusty Task Automator**, a Rust-powered script that reads YAML files and executes predefined tasks. Think of it as a digital assistant that automates routine operations.

## What Can It Do?
- **Automate Tasks**: Run applications, send emails, click buttons, and take screenshots.
- **Read YAML Configurations**: Define automation rules in a structured format.
- **Cross-Platform Compatibility**: Works on Windows, Mac, and Linux.
- **Extensible**: Easily add new task types.
- **Error Logging**: Track issues for debugging.

**or at least that's what it's planned to do**

## Project Structure
```
my_automation_project/
├── Cargo.toml             # Rust package info
└── src/
    ├── main.rs            # Entry point
    ├── config.rs          # Parses YAML configurations
    ├── automation/        # Core automation logic
    │   ├── mod.rs         # Organizes automation modules
    │   ├── triggers.rs    # Defines when to execute tasks
    │   ├── conditions.rs  # Defines execution conditions
    │   └── actions.rs     # Defines task execution logic
    └── utils.rs           # Utility functions
```
**will be changed probably**

## Sample YAML Configuration
```yaml
# automation.yaml
automations:
  - id: "task_1"
    name: "Morning Routine"
    trigger:
      type: "time"
      at: "07:00"
    condition:
      - type: "wifi"
        status: "connected"
    actions:
      - type: "run_app"
        app: "Spotify"
      - type: "wait"
        duration: 10
      - type: "send_email"
        to: "boss@example.com"
        subject: "Running Late"
        body: "Overslept, will be in soon."
```

**will be changed too if needed**

## Summary
Rusty Task Automator simplifies repetitive tasks through automation. Define your workflows in YAML, let Rust handle execution, and streamline your processes.

