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

    let re = Regex::new(r"\[.*\]\(.*\)").unwrap();
    let re_link = Regex::new(r"\(.*\)").unwrap();

    let md_string = steam.unwrap();

    let mut reprasd_md_string = md_string.clone();

    for md_link in re.find_iter(&md_string) {

        let start_index = md_link.start();
        let end_index = md_link.end();
        let match_str = &md_string[start_index..end_index];

        let link_str_match = re_link.find(&match_str).unwrap();
        let list_str_start = link_str_match.start() + 1;
        let list_str_end = link_str_match.end() - 1;

        let link_str = &match_str[list_str_start..list_str_end];

        if link_str.starts_with("http") {
            println!("Http Link: {}", link_str);
        } else {
            let md_without_lowe = link_str.replace(".md", "").to_lowercase();
            let fix_md_link = "../".to_string() + &md_without_lowe;

            reprasd_md_string.replace_range(
                (start_index + list_str_start)..(end_index - 1),
                &fix_md_link,
            );

            println!("MD Link: {}", &fix_md_link);
        }
    }

    fs::write(path, reprasd_md_string).unwrap();
}
