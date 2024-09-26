#[cfg(test)]
mod config_test {
    use rust_platform::config::config::SystemConfig;

    #[test]
    fn test_config() {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("application.yml").expect("not find 'application.yml'");
        let mut cfg_data = "".to_string();
        f.read_to_string(&mut cfg_data)
            .expect("read 'application.yml' fail");
        let data: SystemConfig = serde_yml::from_str(&cfg_data).expect("data must be valid");
        println!("{:?}", data);
    }
}
