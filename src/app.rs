use crate::config::{CliConfig, SaveType};
use easy_rss::RssParser;
use std::io::Write;
use redis::Commands;
use chrono::prelude::*;
use std::ops::AddAssign;
use mysql::prelude::*;


pub struct App<'a,>{
    conf:&'a CliConfig,
    parser: RssParser,
    now:DateTime<Local>,
}

impl<'a> App<'a>{
    pub fn from(conf:&'a CliConfig)->Self{
        Self{
            conf,
            parser:RssParser::new(),
            now:chrono::Local::now()
        }
    }

    pub fn run(&mut self)->Result<(), std::io::Error>{
        self.parser.publish_tag = self.conf.publish_tag.clone();
        self.parser.title_tag = self.conf.title_tag.clone();
        self.parser.link_tag = self.conf.link_tag.clone();
        self.parser.author_tag = self.conf.author_tag.clone();
        self.parser.description_tag = self.conf.description_tag.clone();
        self.parser.guid_tag = self.conf.guid_tag.clone();
        self.parser.publish_tag = self.conf.publish_tag.clone();


        match self.parser.request_xml(
            self.conf.url.as_str(),
            self.conf.charset.as_str()
        ){
            Ok(ret) => {
                self.parser.set_xml(ret);
            },
            Err(_e) =>{
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Failed by request xml."))
            }
        };


        if !self.parser.check_xml() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Failed by check xml."));
        }

        match self.conf.save_type {
            SaveType::Redis =>{
                let data = self.parser.parse_json()?;
                let address = self.conf.driver_url.clone();
                let mut table_name = self.conf.table_name.clone();
                if self.conf.append_date.len() > 0 {
                    table_name.add_assign("_");
                    let now_date = self.now.format(self.conf.append_date.as_str()).to_string();
                    table_name.add_assign(now_date.as_str());
                }

                let redis_client = match redis::Client::open(address) {
                    Ok(con) => con,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,e.to_string()))
                } ;


                let mut connect = match redis_client.get_connection(){
                    Ok(con) => con,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,e.to_string()))
                } ;


                if let Err(e) = connect.set::<String,&[u8],String>(table_name,data.as_bytes()) {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,e.to_string()))
                } ;
            }

            SaveType::File =>{
                let data = self.parser.parse_json()?;
                let mut filename = self.conf.table_name.clone();

                if self.conf.append_date.len() > 0 {
                    filename = filename.replace(".json","");
                    let now_date = self.now.format(self.conf.append_date.as_str()).to_string();
                    filename.add_assign("_");
                    filename.add_assign(now_date.as_str());
                }

                if !filename.contains(".json") {
                    filename.add_assign(".json");
                };

                let fd = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(filename.as_str())?;
                let mut writer = std::io::BufWriter::new(fd);
                writer.write_all(data.as_bytes())?;
            }

            SaveType::MySQL =>{
                let data = self.parser.parse_vec()?;
                let address = self.conf.driver_url.clone();
                let mut table_name = self.conf.table_name.clone();
                if self.conf.append_date.len() > 0 {
                    table_name.add_assign("_");
                    let now_date = self.now.format(self.conf.append_date.as_str()).to_string();
                    table_name.add_assign(now_date.as_str());
                }


                let mysql_client = match mysql::Pool::new(address) {
                    Ok(con) => con,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::NotConnected,e.to_string()))
                } ;
                let mut connect = match mysql_client.get_conn() {
                    Ok(c) => c,
                    Err(e) =>return Err(std::io::Error::new(std::io::ErrorKind::NotConnected,e.to_string()))
                };

                let mut transaction = match connect.start_transaction(mysql::TxOpts::default()) {
                    Ok(c) => c,
                    Err(e) =>return Err(std::io::Error::new(std::io::ErrorKind::NotConnected,e.to_string()))
                };

                let create_sql = format!(r#"CREATE TABLE IF NOT EXISTS `{}` (
                        `uid` char(32) NOT NULL DEFAULT '' COLLATE 'utf8mb4_unicode_ci',
                        `title` varchar(255) NOT NULL DEFAULT '' COLLATE 'utf8mb4_unicode_ci',
                        `link` varchar(255) NOT NULL DEFAULT '' COLLATE 'utf8mb4_unicode_ci',
                        `author` varchar(255) NOT NULL DEFAULT '' COLLATE 'utf8mb4_unicode_ci',
                        `description` LONGTEXT NOT NULL COLLATE 'utf8mb4_unicode_ci',
                        `guid` varchar(255) NOT NULL DEFAULT '' COLLATE 'utf8mb4_unicode_ci',
                        `publish` varchar(50) NOT NULL DEFAULT '' COLLATE 'utf8mb4_unicode_ci',
                        `create_time` int unsigned NOT NULL,
                        PRIMARY KEY (`uid`)
                    )COLLATE=utf8mb4_unicode_ci ENGINE=InnoDB CHARSET=utf8mb4"#,
                    table_name
                );


                if let Err(e) = transaction.query_drop(create_sql) {
                    transaction.rollback().unwrap();
                    return Err(std::io::Error::new(std::io::ErrorKind::NotConnected,e.to_string()));
                }


                let query_sql ="SELECT `uid` FROM `?` WHERE `uid` = '?'";
                data.into_iter().for_each(|val|{

                    let title = val.title.clone();
                    let link = val.link.clone();
                    let author = val.author.clone();
                    let desc = val.description.clone();
                    let guid = val.guid.clone();
                    let publish = val.publish.clone();
                    let create_time = self.now.timestamp().to_string();

                    let uid = if val.guid.len() > 0 {
                        val.guid.clone()
                    }else{
                        val.link.clone()
                    };
                    let uid = format!("{:x}",md5::compute(uid.as_bytes()));
                    let show_uid = uid.clone();

                    let exists = transaction.exec_first::<usize, _, _>(query_sql, (table_name.clone(), uid.clone(),));
                    if exists.is_err(){
                        let insert_sql = format!(
                            r"INSERT INTO `{}` VALUES( ? , ? , ? , ? , ? , ? , ? , ? )",
                            table_name
                        );

                        if let Ok(_)  = transaction.exec_drop(
                            insert_sql, (
                                uid,title,link,author,
                                desc,guid,publish,create_time
                            )
                        ) {
                            println!("INSERT UID = {}",show_uid);
                        }
                    }
                });
            }

            _ => {
                let data = self.parser.parse_json()?;
                let stdout = std::io::stdout();
                let mut writer = std::io::BufWriter::new(stdout);
                writer.write_all(&mut data.as_bytes())?;
            }
        }

        Ok(())
    }
}