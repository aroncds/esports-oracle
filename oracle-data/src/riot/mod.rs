use crate::request::DataRequest;
use reqwest;

struct LeagueOfLegends {
    url: String,

    credentials: String,
}

impl LeagueOfLegends {
    fn new() -> Self {
        Self {
            url: String::from("http://aron."),
            credentials: String::from("asdas")
        }
    }
}

impl DataRequest<String> for LeagueOfLegends {

}