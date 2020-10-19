use crate::config::{OutputType,OutputConfig};
use headless_chrome::protocol::page::{ScreenshotFormat};
use chrono::{DateTime, Local};
use std::ops::AddAssign;
use std::io::Write;


pub struct Output<'a>{
    conf:&'a OutputConfig,
    now:DateTime<Local>,
}

impl<'a> Output<'a>{
    pub fn from(conf:&'a OutputConfig)->Self{
        Self{ conf,now:chrono::Local::now() }
    }



    pub fn run(&mut self)->Result<(), std::io::Error>{
        let chrome_path = std::path::PathBuf::from(
            self.conf.chrome_path.as_str()
        );

        let headless = self.conf.headless;
        let sandbox = self.conf.sandbox;
        let window_size_width = self.conf.window_size_width;
        let window_size_height = self.conf.window_size_height;
        let idle_browser_timeout = self.conf.idle_browser_timeout;

        let browser_builder = headless_chrome::LaunchOptionsBuilder::default()
            .path(Some(chrome_path))
            .headless(headless)
            .sandbox(sandbox)
            .idle_browser_timeout(idle_browser_timeout)
            .window_size(Some((window_size_width,window_size_height)))
            .build()
            .unwrap();


        let list = vec![
            "https://www.baidu.com",
            "https://cn.bing.com",
            "https://www.meteorcat.com/#/"
        ];


        match headless_chrome::Browser::new(browser_builder) {
            Ok(browser) =>{
                match browser.wait_for_initial_tab() {
                    Ok(tab) =>{

                        list.iter().for_each(|url|{
                            let url_str = url.to_string();
                            let hash_name = format!("{:x}",md5::compute(url_str.as_bytes()));
                            let pathname = self.conf.output_path.clone();

                            if let Err(e) = tab.navigate_to(url_str.as_str()) {
                                eprintln!("{}",e.to_string());
                                return ;
                            }

                            let mut filename = if self.conf.append_date.len() > 0 {
                                let now_date = self.now.format(self.conf.append_date.as_str()).to_string();
                                let path = format!("{}/{}",pathname,now_date);

                                if !std::path::Path::new(path.as_str()).exists() {
                                    let pathname_clone = path.clone();
                                    std::fs::create_dir_all(pathname_clone.as_str()).unwrap();
                                };

                                format!("{}/{}/{}",pathname,now_date,hash_name)
                            }else{

                                if !std::path::Path::new(pathname.as_str()).exists() {
                                    let pathname_clone = pathname.clone();
                                    std::fs::create_dir_all(pathname_clone.as_str()).unwrap();
                                };
                                format!("{}/{}",pathname,hash_name)
                            };


                            match self.conf.output_type {
                                OutputType::PDF => {
                                    match tab.print_to_pdf(None) {
                                        Ok(pdf) =>{
                                            filename.add_assign(".pdf");
                                            let mut fd = std::fs::OpenOptions::new()
                                                .write(true)
                                                .create(true)
                                                .open(filename)
                                                .unwrap();

                                            fd.write_all(pdf.as_slice()).unwrap();
                                        }

                                        Err(e)=>{
                                            eprintln!("{}",e.to_string());
                                        }

                                    }
                                }

                                OutputType::PNG => {
                                    match tab.capture_screenshot(
                                        ScreenshotFormat::PNG,
                                        None,
                                        true) {
                                        Ok(png) =>{
                                            filename.add_assign(".png");
                                            let mut fd = std::fs::OpenOptions::new()
                                                .write(true)
                                                .create(true)
                                                .open(filename)
                                                .unwrap();

                                            fd.write_all(png.as_slice()).unwrap();
                                        }

                                        Err(e)=>{
                                            eprintln!("{}",e.to_string());
                                        }

                                    }
                                }

                                OutputType::JPEG(quality) =>{
                                    match tab.capture_screenshot(
                                        ScreenshotFormat::JPEG(Some(quality)),
                                        None,
                                        true) {

                                        Ok(jpeg) => {
                                            filename.add_assign(".jpeg");
                                            let mut fd = std::fs::OpenOptions::new()
                                                .write(true)
                                                .create(true)
                                                .open(filename)
                                                .unwrap();

                                            fd.write_all(jpeg.as_slice()).unwrap();
                                        }

                                        Err(e) =>{
                                            eprintln!("{}",e.to_string());
                                        }
                                    }
                                }

                                _ => {},
                            }

                        });
                    },
                    Err(e)=>{
                        return Err(std::io::Error::new(std::io::ErrorKind::Other,e.to_string()))
                    }
                }

                /*
                headless_chrome::util::Wait::new(
                    self.conf.idle_browser_timeout,
                    self.conf.idle_browser_timeout,
                );
                 */
            }
            Err(e) =>{
                return Err(std::io::Error::new(std::io::ErrorKind::Other,e.to_string()))
            }
        }
        Ok(())
    }
}