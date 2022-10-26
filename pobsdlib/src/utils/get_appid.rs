pub fn get_app_id(url: &str) -> Option<&str> {
    let url = url.split("app/").collect::<Vec<&str>>();
    let last_part = url.get(1);
    if let Some(last_part) = last_part {
        if let Some(appid) = last_part.split('/').collect::<Vec<&str>>().first() {
            Some(appid)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test_get_app_id {
    use super::*;
    #[test]
    fn regular_url() {
        let url = "https://store.steampowered.com/app/245390/I_Have_No_Mouth_and_I_Must_Scream/";
        assert_eq!(get_app_id(url).unwrap(), "245390");
    }
    #[test]
    fn without_name_url() {
        let url = "https://store.steampowered.com/app/245390";
        assert_eq!(get_app_id(url).unwrap(), "245390");
    }
    #[test]
    fn broken_url() {
        let url = "https://store.steampowered.com/";
        assert_eq!(get_app_id(url), None);
    }
}
