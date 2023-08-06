use std::{str::FromStr, sync::{Arc, Mutex}, collections::HashMap};

use reqwest::Method;
use crate::client::Request;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Client{    
    http_client: reqwest::Client,
}

impl Client {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
        }
    }

    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new()
        }
    }

    async fn send(&mut self, bar: Arc<Mutex<ProgressBar>>, req: Request, req_count: u32, resp_stats: &mut Arc<Mutex<HashMap<String, u32>>>) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..req_count {
            let resp = self.http_client.
            request(Method::from_str(req.method.as_str()).unwrap(), req.path.as_str()).
            send().await;
            
            match resp {
                Ok(resp) => {
                    let status = resp.status().as_str().to_string();
                    let mut stats = resp_stats.lock().unwrap();
                    let count = stats.entry(status).or_insert(0);
                    *count += 1;
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
        
        let req_count_per_thread = req_count / thread_count as u32;

        let bar = Arc::new(Mutex::new(ProgressBar::new(req_count as u64)));
        bar.lock().unwrap().set_style(
            ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap(),
        );

        let resp_stats: Arc<Mutex<std::collections::HashMap<String, u32>>> = Arc::new(Mutex::new(std::collections::HashMap::new()));

        let mut handles = vec![];
        for _ in 0..thread_count {
            let req = req.clone();
            let mut client = self.clone();

            let bar_clone = Arc::clone(&bar);
            let mut resp_stats_clone = Arc::clone(&resp_stats);

            let handle = tokio::spawn(async move {
                client.send(bar_clone, req, req_count_per_thread, &mut resp_stats_clone).await.unwrap();
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        bar.lock().unwrap().finish_with_message("Done");

        return self.stats(resp_stats.lock().unwrap().clone());
    }
        
    fn stats(&self, resp_stats: std::collections::HashMap<String, u32>) -> String {
        let mut stats_str: String = String::new();

        for (status, count) in &resp_stats {
            stats_str.push_str(&format!("{}: {} ", status, count));
        }

        stats_str
    }
}
