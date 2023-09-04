use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    println!("Path: {}", path);

    let steam = fs::read_to_string(path);

    if let Err(e) = steam {
        println!("Error: {}", e);
        return;
    }

    let re = Regex::new(r"\[[^\]]*\]\([^\)]*\)").unwrap();
    let re_link = Regex::new(r"\(.*\)").unwrap();

    let md_string = steam.unwrap();

    let mut reprised_md_string = md_string.clone();

    for md_link in re.find_iter(&md_string) {
        let start_index = md_link.start();
        let end_index = md_link.end();
        let match_str = &md_string[start_index..end_index];

        let link_str_match = re_link.find(&match_str).unwrap();
        let list_str_start = link_str_match.start() + 1;

        let link_str = &match_str[list_str_start..match_str.len() - 1];

        if link_str.starts_with("http") {
            println!("Http Link: {}", link_str);
        } else {
            let mut md_without_lowe = link_str.replace(".md", "");
            if md_without_lowe.starts_with("Manual") {
                md_without_lowe = md_without_lowe
                    .replace("Manual", "")
                    .replace("/JP/", "")
                    .replace("/EN/", "");
            }
            md_without_lowe = md_without_lowe.to_lowercase();
            let fix_md_link = "../".to_string() + &md_without_lowe;

            let mut reprasd_match_str = match_str.to_string();
            reprasd_match_str.replace_range(list_str_start..match_str.len() - 1, &fix_md_link);

            reprised_md_string = reprised_md_string.replace(&match_str, &reprasd_match_str);

            println!("MD Link: {}", &fix_md_link);
        }
    }

    fs::write(path, reprised_md_string).unwrap();
}
