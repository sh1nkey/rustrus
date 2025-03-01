
mod text_check_workers;
use text_check_workers::text_check_worker_mltr;


fn main() {
    let text = "пиздаблять";
    let chosen_categories: Vec<String> = vec!["sexual".to_string(), "strong".to_string()];
    let res = text_check_worker_mltr(text, chosen_categories);
    if res != None {
        println!("is {}", res.unwrap());
    }
}