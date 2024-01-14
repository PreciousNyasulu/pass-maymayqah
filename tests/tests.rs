#[cfg(test)]
mod tests {
    use pass_maymayqah::utils;

    #[test]
    fn test_create_directory() {
        let mut folder_path = std::env::var("HOME").unwrap();
        folder_path.push_str("/pass_maymayqah");
    
        let result = utils::create_program_folder(&folder_path);
        assert_eq!(result, true);
    }
    
}
