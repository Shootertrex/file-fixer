use regex::Regex;
use std::env;
use std::fs;

/// Formats files to be Windows compliant plus a few extra rules
/// Rules:
/// < > : " / \ | ? * are illegal characters
/// Multiple periods (.) are allowed
/// No spaces ( ) despite Windows supporting them. They are cumbersome to use with terminals

pub fn main() {
    let args: Vec<String> = env::args().collect();

    format(&args);
}

fn format(args: &[String]) {
    let directory = get_directory(&args);

    let files = fs::read_dir(directory).unwrap_or_else(|error| {
        println!("Error: {}", error);
        std::process::exit(1);
    });

    let mut counter = 0;

    for file in files {
        let original_path = file.unwrap().path();
        let original_name = original_path.file_name().unwrap().to_str().unwrap();

        let mut new_path = original_path.clone();
        new_path.pop();

        let re = Regex::new(r#"[<>:"/\\|?\* ]"#).unwrap();
        let new_name = re.replace_all(original_name, "_").into_owned();
        new_path.push(new_name);

        match fs::rename(&original_path, &new_path) {
            Ok(_) => counter += 1,
            Err(error) => println!("Error: {}", error),
        }
    }
    println!("Total files modified: {}", counter);
}

fn get_directory(args: &[String]) -> String {
    if args.len() == 2 {
        args[1].clone()
    } else {
        env::current_dir()
            .unwrap_or_else(|error| {
                println!("Error: {}", error);
                std::process::exit(1);
            })
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::format;
    use std::fs;
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn ensure_delete_file_deletes_it() {
        // given
        let dir = TempDir::new("unit_test").unwrap();
        let good_file = "good_file.txt";
        let colon_split = "colon_split.jpg:large.jpg";
        let space_split = "space_split.jpg large.jpg";
        let middle_space = "middle space.jpg";
        File::create(dir.path().join(good_file)).unwrap();
        File::create(dir.path().join(colon_split)).unwrap();
        File::create(dir.path().join(space_split)).unwrap();
        File::create(dir.path().join(middle_space)).unwrap();
        let test_directory = dir.path().to_str().unwrap();

        // when
        format(&vec!["".to_owned(), test_directory.to_owned()]);

        // then
        assert!(fs::read(dir.path().join(good_file)).is_ok());
        assert!(fs::read(dir.path().join(colon_split)).is_err());
        assert!(fs::read(dir.path().join(space_split)).is_err());
        assert!(fs::read(dir.path().join(middle_space)).is_err());

        let expected_colon_split = "colon_split.jpg_large.jpg";
        let expected_space_split = "space_split.jpg_large.jpg";
        let expected_middle_space = "middle_space.jpg";
        assert!(fs::read(dir.path().join(expected_colon_split)).is_ok());
        assert!(fs::read(dir.path().join(expected_space_split)).is_ok());
        assert!(fs::read(dir.path().join(expected_middle_space)).is_ok());
    }
}
