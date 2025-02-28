use std::fs;
use phf::phf_map;

//use rustrict::CensorStr;


static FILES: phf::Map<&'static str, &'static str> = phf_map! {
    "sexual" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\sexual.txt",
    "strong" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\stongswords.txt",
    //"political" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\political.txt",
};

fn main() {
    let text = "хуеверт";
    let chosen_categories: Vec<&str> = vec!["sexual", "strong"];
    text_check_worker(text, &chosen_categories);
}


fn text_check_worker(text: &str, chosen_categories: &Vec<&str>) {
    let words: &Vec<&str> = &text.split_whitespace().collect();
    for category in chosen_categories {
        if let Some(file_path) = FILES.get(category) {
            if check_text(&words, file_path.to_string()) {
                println!("is {}", category)
            }
        }
    }
}



fn check_text(words: &Vec<&str>, file_name: String) -> bool {
    for word in words {
        if check_in_file(word.to_string(), &file_name) {
            return true
        }
    }
    return false
}

fn check_in_file(word: String, file_name: &String) -> bool {
    let file_contents = fs::read_to_string(file_name).expect("Unable to read file");

    for line in file_contents.lines() {
        if word.contains(line) {
            return true
        }
    }
    return false
}