use prettychars::glyph;

// Example: Creates a system monitoring dashboard with progress bars, service
// status indicators, sparkline charts, and alerts using heavy box drawing
// and gradient block characters. Run with: cargo run --example dashboard

fn main() {
    draw_dashboard();
}

fn draw_dashboard() {
    // Box drawing
    let tl = glyph("box.heavy.tl").unwrap();
    let tr = glyph("box.heavy.tr").unwrap();
    let bl = glyph("box.heavy.bl").unwrap();
    let br = glyph("box.heavy.br").unwrap();
    let h = glyph("box.heavy.h").unwrap();
    let v = glyph("box.heavy.v").unwrap();
    let _t_down = glyph("box.heavy.t-down").unwrap();
    let _t_up = glyph("box.heavy.t-up").unwrap();
    let t_right = glyph("box.heavy.t-right").unwrap();
    let t_left = glyph("box.heavy.t-left").unwrap();
    let cross = glyph("box.heavy.cross").unwrap();

    // Block characters for progress bars
    let blocks = [
        glyph("block.lower.1").unwrap(),
        glyph("block.lower.2").unwrap(),
        glyph("block.lower.3").unwrap(),
        glyph("block.lower.4").unwrap(),
        glyph("block.lower.5").unwrap(),
        glyph("block.lower.6").unwrap(),
        glyph("block.lower.7").unwrap(),
        glyph("block.full").unwrap(),
    ];

    // Status symbols
    let check = glyph("check.heavy").unwrap();
    let cross_mark = glyph("check.x.heavy").unwrap();
    let warning = glyph("misc.warning").unwrap();
    let arrow_up = glyph("arrow.up").unwrap();
    let arrow_down = glyph("arrow.down").unwrap();

    // Title
    println!("\n");
    draw_line(&tl, &h, &tr, 60);
    println!("{} {:^58} {}", v, "System Dashboard", v);
    draw_separator(&t_right, &h, &cross, &t_left, 60);

    // CPU Usage
    println!("{} CPU Usage:  {:>3}%  {}", v, 73, v);
    print!("{} ", v);
    draw_progress_bar(73, 50, &blocks);
    println!(" {}", v);
    
    // Memory
    println!("{} Memory:     {:>3}%  {}", v, 45, v);
    print!("{} ", v);
    draw_progress_bar(45, 50, &blocks);
    println!(" {}", v);
    
    // Disk
    println!("{} Disk:       {:>3}%  {}", v, 89, v);
    print!("{} ", v);
    draw_progress_bar(89, 50, &blocks);
    println!(" {}", v);

    draw_separator(&t_right, &h, &cross, &t_left, 60);

    // Services status
    println!("{} Services:                                                {}", v, v);
    println!("{} {} nginx        {} apache      {} postgresql            {}", v, check, check, cross_mark, v);
    println!("{} {} redis        {} mongodb     {} mysql                 {}", v, check, cross_mark, check, v);

    draw_separator(&t_right, &h, &cross, &t_left, 60);

    // Metrics with sparklines
    println!("{} Network Traffic:                                         {}", v, v);
    print!("{} {} Upload:   ", v, arrow_up);
    draw_sparkline(&[3, 5, 4, 6, 8, 7, 9, 10, 8, 11, 9, 12], &blocks);
    println!("  2.3 MB/s  {}", v);
    
    print!("{} {} Download: ", v, arrow_down);
    draw_sparkline(&[8, 9, 7, 10, 12, 11, 15, 14, 13, 16, 15, 18], &blocks);
    println!("  8.7 MB/s  {}", v);

    draw_separator(&t_right, &h, &cross, &t_left, 60);

    // Alerts
    println!("{} Alerts:                                                  {}", v, v);
    println!("{} {} High CPU usage detected                              {}", v, warning, v);
    println!("{} {} Disk space running low                               {}", v, warning, v);

    draw_line(&bl, &h, &br, 60);
    println!();
}

fn draw_line(left: &str, middle: &str, right: &str, width: usize) {
    print!("{}", left);
    for _ in 0..width-2 {
        print!("{}", middle);
    }
    println!("{}", right);
}

fn draw_separator(left: &str, middle: &str, cross: &str, right: &str, width: usize) {
    print!("{}", left);
    for i in 0..width-2 {
        if i > 0 && i % 20 == 0 {
            print!("{}", cross);
        } else {
            print!("{}", middle);
        }
    }
    println!("{}", right);
}

fn draw_progress_bar(percent: u8, width: usize, blocks: &[&str]) {
    let filled = (percent as usize * width * 8) / 100;
    let full_blocks = filled / 8;
    let partial = filled % 8;

    print!("[");
    for i in 0..width {
        if i < full_blocks {
            print!("{}", blocks[7]);
        } else if i == full_blocks && partial > 0 {
            print!("{}", blocks[partial - 1]);
        } else {
            print!(" ");
        }
    }
    print!("]");
}

fn draw_sparkline(values: &[u8], blocks: &[&str]) {
    let max = *values.iter().max().unwrap_or(&1);
    for &val in values {
        let index = ((val as usize * 7) / max as usize).min(7);
        print!("{}", blocks[index]);
    }
}
