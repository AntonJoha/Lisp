use super::utils;
use std::collections::VecDeque;
use std::fs;



fn get_result(result: String) -> String {
     fs::read_to_string(result)
        .expect("Could not read file").to_string()

}

fn get_files() -> Vec<String>{


    let files = fs::read_dir("example_code").unwrap();

    let mut paths: Vec<String> = Vec::new();
        
    for f in files {
        let path = f.unwrap().path().to_str().unwrap().to_string();
        paths.push(path.split(".").next().unwrap().to_string());
    }
    paths.sort();
    paths.dedup();
    paths
}

#[test]
fn test_file() {

    let files = get_files();

    for file in files {
        let input = file.clone() + ".lisp";
        let result = file.clone() + ".result";

        let v = utils::test_file(input);
        let result = get_result(result);
        
        assert_eq!(file.clone() + " " + v.literal.as_str(),
            file.clone() + " " + &result[0..result.len()-1]
            );
    }

}

