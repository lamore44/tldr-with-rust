use colored::Colorize;

pub fn print_page(content: &str){
    println!();

    for line in content.lines(){
        print_line(line);
    }
    println!();
}

fn print_line(line: &str){
    if line.trim().is_empty(){
        println!();
        return;

    }

    //judul => putih tebal
    if let Some(title) = line.strip_prefix("# "){
        println!("{}", title.bold().white());
        return;
    }

    //desc => italic
    if let Some(desc) = line.strip_prefix("> "){
        println!("{}", desc.italic().truecolor(180, 180, 180));
        return;
    }

    //explanation => green
    if let Some(label) = line.strip_prefis("- "){
        let label = label.trim_end_matches(':');
        println!("{}", label.green());
        return;
    }

    //command placeholder => warna cyan tanpa highlight placeholder
    if line.trim_start().starts_with('`') && line.trim_end().ends_with('`'){
        let inner = line.trim().trim_matches('`');
        let formatted = format_command(inner);
        println!("    {}", formatted);
        return;
    }

    //yang lainnya tinggal print kayak biasa
    println!("{}", line);
}

fn format_command(command: &str) -> String{
    let mut result = String::new();
    let mut remaining = command;
    
    while let Some(start) = remaining.find("{{"){
        let before = &remaining[..start];
        result.push_str(&before.cyan().to_string());

        let after_open = &remaining[start + 2..];
        if let Some(end) = after_open.find("}}"){
            let placeholder  = &after_open[..end];
            //highlight placeholder warna kuning
            result.push_str(&format!("{{{{{}}}}}", placeholder).bold().yellow().to_string());
            remaining = &after_open[end + 2..];
    
        }else{
            result.push_str(&remaining[start..].cyan().to_string());
            return result;
        }
    }

    result.push_str(&remaining.cyan().to_string());
    result
}