#[cfg(test)]
mod test {
    use actix_web::rt::Runtime;
    use rust_platform::common::utils::address_util::{get_ip_address, Address};
    use serde_json::json;

    #[test]
    fn test() -> Result<(), reqwest::Error> {
        let response = reqwest::blocking::get(
            "http://whois.pconline.com.cn/ipJson.jsp?ip=117.25.169.123&json=true",
        );
        let obj = response?.text()?;
        let objStr = serde_json::from_str::<Address>(&obj).unwrap();
        println!("obj:{:?}", json!(&objStr));

        let th = Runtime::new().unwrap().block_on(async {
            let string = get_ip_address("117.25.169.123").await.unwrap();
            println!("string:{}", string);
        });
        // while th.is_finished(){
        //   return  Ok(());
        // }
        Ok(())
    }
}
