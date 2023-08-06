use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author="msrexe", about="simple api load testing tool", version="0.1.0")]
pub struct AppArgs {
    #[clap(short='u', long="url", required=true)]
    pub url: String,

    #[clap(short='r', long="req", required = true)]
    pub req_count: u32,

    #[clap(short='m', long="method", default_value = "GET")]
    pub method: String,

    #[clap(short='t', long="threads", default_value = "1")]
    pub threads: u8,

    #[clap(short='b', long="body", default_value = "")]
    pub body: String,

    #[clap(short='h', long="headers", default_value = "")]
    pub headers: String,
}
