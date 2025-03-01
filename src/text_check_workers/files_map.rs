use phf::phf_map;

pub static FILES: phf::Map<&'static str, &'static str> = phf_map! {
    "sexual" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\filter_data\sexual.txt",
    "strong" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\filter_data\stongswords.txt",
    //"political" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\political.txt",
};