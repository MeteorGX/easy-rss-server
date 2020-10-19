use easy_rss::{RSS_DEFAULT_TITLE_TAG, RSS_DEFAULT_LINK_TAG, RSS_DEFAULT_AUTHOR_TAG, RSS_DEFAULT_DESC_TAG, RSS_DEFAULT_GUID_TAG, RSS_DEFAULT_PUBLISH_TAG};

#[derive(Debug,Clone,PartialEq)]
pub enum SaveType{
    None,
    File,
    Redis,
    MySQL,
}

#[derive(Debug,Clone)]
pub struct CliConfig{
    pub url: String,
    pub charset: String,
    pub save_type: SaveType,
    pub driver_url: String,
    pub table_name: String,
    pub append_date: String,

    pub title_tag:String,
    pub link_tag:String,
    pub author_tag: String,
    pub description_tag:String,
    pub guid_tag:String,
    pub publish_tag:String,
}


#[derive(Debug,Clone,PartialEq)]
pub enum OutputType{
    None,
    JPEG(u32),
    PNG,
    PDF,
}


#[derive(Debug,Clone)]
pub struct OutputConfig{
    pub chrome_path: String,
    pub headless: bool,
    pub sandbox: bool,
    pub idle_browser_timeout: std::time::Duration,
    pub window_size_width: u32,
    pub window_size_height: u32,
    pub output_type: OutputType,
    pub output_path: String,
    pub append_date: String
}

fn throw_err(e:&str)->std::io::Error{
    return std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        e
    );
}


impl CliConfig{

    pub fn from<'a>(filename:&str)->Result<Self,std::io::Error>{
        let json_config = std::fs::read_to_string(filename)?;
        let conf = match json::parse(json_config.as_str()) {
            Ok(c)=>c,
            Err(e) => {
                return Err(throw_err(e.to_string().as_str()));
            }
        };


        if !conf.is_object() {
            return Err(throw_err("Failed by Config[Json File]."));
        }

        if !conf.has_key("url") && !conf["url"].is_string()  {
            return Err(throw_err("Failed by Config[url]."));
        }

        if !conf.has_key("charset") && !conf["charset"].is_string() {
            return Err(throw_err("Failed by Config[charset]."));
        }

        if !conf.has_key("save_type") && !conf["save_type"].is_string() {
            return Err(throw_err("Failed by Config[save_type]."));
        }

        let save_type_str = conf["save_type"].to_string().to_lowercase();
        let save_type = match save_type_str.as_str() {
            "redis" => SaveType::Redis,
            "mysql" => SaveType::MySQL,
            "file" => SaveType::File,
            _ => SaveType::None
        };


        let driver_url = if  save_type != SaveType::None &&
            conf.has_key("driver_url") &&
            conf["driver_url"].is_string() {
            conf["driver_url"].to_string()
        }else {
            String::new()
        };

        let table_name = if conf.has_key("table_name") &&
            conf["table_name"].is_string() {
            conf["table_name"].to_string()
        }else if save_type != SaveType::None {
            return Err(throw_err("Failed by Config[table_name]."));
        }else{
            String::new()
        };

        let append_date = if  save_type != SaveType::None &&
            conf.has_key("append_date") &&
            conf["append_date"].is_string() {
            conf["append_date"].to_string()
        }else {
            String::new()
        };



        let title_tag = if !conf.has_key("title_tag") &&
            !conf["title_tag"].is_string() {
            RSS_DEFAULT_TITLE_TAG.to_string()
        }else{
            conf["title_tag"].to_string()
        };

        let link_tag = if !conf.has_key("link_tag") &&
            !conf["link_tag"].is_string() {
            RSS_DEFAULT_LINK_TAG.to_string()
        }else{
            conf["link_tag"].to_string()
        };

        let author_tag = if !conf.has_key("author_tag") &&
            !conf["author_tag"].is_string() {
            RSS_DEFAULT_AUTHOR_TAG.to_string()
        }else{
            conf["author_tag"].to_string()
        };

        let description_tag = if !conf.has_key("description_tag") &&
            !conf["description_tag"].is_string() {
            RSS_DEFAULT_DESC_TAG.to_string()
        }else{
            conf["description_tag"].to_string()
        };


        let guid_tag = if !conf.has_key("guid_tag") &&
            !conf["guid_tag"].is_string() {
            RSS_DEFAULT_GUID_TAG.to_string()
        }else{
            conf["guid_tag"].to_string()
        };

        let publish_tag = if !conf.has_key("publish_tag") &&
            !conf["publish_tag"].is_string() {
            RSS_DEFAULT_PUBLISH_TAG.to_string()
        }else{
            conf["publish_tag"].to_string()
        };


        Ok(Self{
            url: conf["url"].to_string(),
            charset: conf["charset"].to_string(),

            save_type,
            driver_url,
            table_name,
            append_date,

            title_tag,
            link_tag,
            author_tag,
            description_tag,
            guid_tag,
            publish_tag,
        })
    }
}


impl OutputConfig {
    pub fn from<'a>(filename: &str) -> Result<Self, std::io::Error> {
        let json_config = std::fs::read_to_string(filename)?;
        let conf = match json::parse(json_config.as_str()) {
            Ok(c) => c,
            Err(e) => {
                return Err(throw_err(e.to_string().as_str()));
            }
        };

        if !conf.is_object() {
            return Err(throw_err("Failed by Config[Json File]."));
        }

        if !conf.has_key("chrome_path") && !conf["chrome_path"].is_string()  {
            return Err(throw_err("Failed by Config[url]."));
        }

        if !conf.has_key("headless") && !conf["headless"].is_boolean()  {
            return Err(throw_err("Failed by Config[headless]."));
        }

        if !conf.has_key("sandbox") && !conf["sandbox"].is_boolean()  {
            return Err(throw_err("Failed by Config[sandbox]."));
        }

        if !conf.has_key("idle_browser_timeout") && !conf["idle_browser_timeout"].is_number()  {
            return Err(throw_err("Failed by Config[idle_browser_timeout]."));
        }

        if !conf.has_key("window_size_width") && !conf["window_size_width"].is_number()  {
            return Err(throw_err("Failed by Config[window_size_width]."));
        }

        if !conf.has_key("window_size_height") && !conf["window_size_height"].is_number()  {
            return Err(throw_err("Failed by Config[window_size_height]."));
        }




        if !conf.has_key("output_type") && !conf["output_type"].is_string() {
            return Err(throw_err("Failed by Config[output_type]."));
        }

        let output_type_str = conf["output_type"].to_string().to_lowercase();
        let output_type = match output_type_str.as_str() {
            "jpeg" => OutputType::JPEG(
                if !conf.has_key("output_quality") && !conf["output_quality"].is_number(){
                    100
                }else {
                    conf["output_quality"].as_u32().unwrap()
                }
            ),
            "png" => OutputType::PNG,
            "pdf" => OutputType::PDF,
            _ => OutputType::None
        };


        let output_path = if !conf.has_key("output_path") &&
            !conf["output_path"].is_string() {
            String::from(".")
        }else{
            conf["output_path"].to_string()
        };

        let append_date = if !conf.has_key("append_date") &&
            !conf["append_date"].is_string() {
            String::new()
        }else{
            conf["append_date"].to_string()
        };



        Ok(Self {
            chrome_path:conf["chrome_path"].to_string(),
            headless: conf["headless"].as_bool().unwrap(),
            sandbox: conf["sandbox"].as_bool().unwrap(),
            idle_browser_timeout: std::time::Duration::new(conf["idle_browser_timeout"].as_u64().unwrap(),0),
            window_size_width: conf["window_size_width"].as_u32().unwrap(),
            window_size_height: conf["window_size_height"].as_u32().unwrap(),
            output_type,
            output_path,
            append_date
        })
    }
}