use prettychars::{glyph, style, Style};

fn main() {
    println!("=== Chaining Examples ===\n");

    // Method 1: Direct chaining with string building
    let title = format!("{} {} {} {}", 
        glyph("star").unwrap(),
        style("PRETTYCHARS", Style::MathBold),
        style("DEMO", Style::Circled),
        glyph("star").unwrap()
    );
    println!("Title: {}\n", title);

    // Method 2: Building complex UI elements
    let progress_line = format!(
        "{} Progress: [{}{}{}] {}% {}",
        glyph("arrow.right").unwrap(),
        glyph("block.full").unwrap().repeat(7),
        glyph("block.left.4").unwrap(),
        " ".repeat(5),
        75,
        glyph("check.mark").unwrap()
    );
    println!("Progress: {}\n", progress_line);

    // Method 3: Status indicators
    let services = [
        ("nginx", true),
        ("redis", true), 
        ("postgres", false),
        ("mongodb", true),
    ];

    println!("Services:");
    for (service, running) in services {
        let status_icon = if running { 
            glyph("check.heavy").unwrap() 
        } else { 
            glyph("check.x.heavy").unwrap() 
        };
        let service_name = style(service, Style::Monospace);
        println!("  {} {}", status_icon, service_name);
    }
    println!();

    // Method 4: Mixed styling within text
    let mixed_text = format!(
        "Welcome to {} {}! {} {}",
        style("Rust", Style::MathBold),
        style("2024", Style::Superscript),
        glyph("misc.lightning").unwrap(),
        style("Happy coding!", Style::Script)
    );
    println!("Mixed: {}\n", mixed_text);

    // Method 5: Building tables with alignment
    let table_data = [
        ("CPU", "73%", "High"),
        ("Memory", "45%", "OK"),
        ("Disk", "89%", "Warning"),
    ];

    println!("System Status:");
    for (metric, value, status) in table_data {
        let icon = match status {
            "High" | "Warning" => glyph("misc.warning").unwrap(),
            _ => glyph("check.mark").unwrap(),
        };
        let styled_metric = style(metric, Style::SansSerifBold);
        let styled_value = style(value, Style::Monospace);
        println!("  {} {:<8} {:<8} {}", 
            icon, styled_metric, styled_value, status);
    }
    println!();

    // Method 6: Complex dashboard element
    let dashboard_header = format!(
        "{}{}{} {} {}{}{}", 
        glyph("box.heavy.tl").unwrap(),
        glyph("box.heavy.h").unwrap().repeat(20),
        glyph("box.heavy.tr").unwrap(),
        style("DASHBOARD", Style::SmallCaps),
        glyph("box.heavy.tl").unwrap(),
        glyph("box.heavy.h").unwrap().repeat(20),
        glyph("box.heavy.tr").unwrap()
    );
    println!("Dashboard: {}\n", dashboard_header);

    // Method 7: Function chaining helper
    fn build_alert(level: &str, message: &str) -> String {
        let (icon, styled_level) = match level {
            "error" => (glyph("check.x.heavy").unwrap(), style("ERROR", Style::MathBold)),
            "warn" => (glyph("misc.warning").unwrap(), style("WARN", Style::Squared)),
            "info" => (glyph("check.mark").unwrap(), style("INFO", Style::Circled)),
            _ => ("?", level.to_string()),
        };
        
        format!("{} {} {}", icon, styled_level, message)
    }

    println!("Alerts:");
    println!("  {}", build_alert("error", "Database connection failed"));
    println!("  {}", build_alert("warn", "High memory usage detected"));
    println!("  {}", build_alert("info", "System backup completed"));
}