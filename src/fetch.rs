const BASE_URL: &str = "https://raw.githubusercontent.com/tldr-pages/tldr/main/pages";

pub fn get_page(command: &str, platform: &str) -> Result<String, String>{
    //coba fetch platform spesifik (linux, osx, windows)
    if platform != "common"{
        if let Ok(content) = fetch_url(command, platform){
            Ok(content);
        } 
    }
    fetch_url(command, "common") //fallback ke common (tempat semua command)
}

fn fetch_url(command: &str, platform: &str) -> Result<String, String>{
    let url = format!("{}/{}/{}.md", BASE_URL, platform, command);
    let response = reqwest::blocking::get(&url).map_err(|e| format!("Network Err: {}", e));

    if response.status().is_success(){
        response.text().map_err(|e| format!("Failed to read responses: {}", e))?;
    }else{
        Err(format!("Page not found (HTTP {}): {}", response.status(), url))
    }
}