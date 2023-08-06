use std::{str::FromStr, sync::{Arc, Mutex}};

use reqwest::Method;
use crate::client::Request;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Client{
    resp_stats: std::collections::HashMap<String, u32>,
    
    http_client: reqwest::Client,
}

impl Client {
    fn clone(&self) -> Self {
        Self {
            resp_stats: self.resp_stats.clone(),
            http_client: self.http_client.clone(),
        }
    }

    pub fn new() -> Self {
        Self {
            resp_stats: std::collections::HashMap::new(),
            http_client: reqwest::Client::new()
        }
    }

    async fn send(&mut self, bar:Arc<Mutex<ProgressBar>>, req: Request, req_count: u32) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..req_count {
            let resp = self.http_client.
            request(Method::from_str(req.method.as_str()).unwrap(), req.path.as_str()).
            send().await;
            
            match resp {
                Ok(resp) => {
                    let status = resp.status().as_str().to_string();
                    let count = self.resp_stats.entry(status.clone()).or_insert(0);
                    *count += 1;
                    
                    print!("{}-{}\n", status, count);

                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
            
            bar.lock().unwrap().inc(1);
        }

        Ok(())
    }   

    pub async fn start(&mut self, req: Request, thread_count:u8, req_count: u32) -> String {
        println!("Starting client... \nthreads: {}, request count: {}, method: {}, target: {}\n", thread_count, req_count, req.method.as_str(), req.path);

        // let _ = self.send(req, req_count).await;
        
        let req_count_per_thread = req_count / thread_count as u32;
        // let bar = ProgressBar::new(req_count_per_thread as u64);
        // bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap());

        let bar = Arc::new(Mutex::new(ProgressBar::new(req_count as u64)));
        bar.lock().unwrap().set_style(
            ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap(),
        );

        let mut handles = vec![];
        for _ in 0..thread_count {
            let req = req.clone();
            let mut client = self.clone();
            let bar_clone = Arc::clone(&bar);
            let handle = tokio::spawn(async move {
                client.send(bar_clone, req, req_count_per_thread).await.unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        bar.lock().unwrap().finish_with_message("Done");
        return self.stats();
    }
        
    fn stats(&self) -> String {
        let mut stats: String = String::new();

        for (status, count) in &self.resp_stats {
            print!("{}: {} ", status, count);
            stats.push_str(&format!("{}: {} ", status, count));
        }

        print!("stats: {}", stats);
        stats
    }
}
