#![allow(unused)]

use miniserde::{json, Serialize, Deserialize};

#[derive(Deserialize)]
struct Commenter {
    display_name: String,
}


#[derive(Deserialize)]
struct Comment {
    commenter: Commenter,
    message: Message,
    updated_at: String,
}


#[derive(Deserialize)]
struct Message {
    body: String,
}


#[derive(Deserialize)]
struct Comments {
    comments : Vec<Comment>,
}

#[derive(argh::FromArgs)]
/// Convert rechat JSON to srt subtitles
/// Feed JSON to stdin, get srt on stdout
struct Opts {
    /// filenames of input JSON chunks, in order
    #[argh(positional)]
    files: Vec<std::path::PathBuf>,

    /// unix timestamp of the beginning of the video
    #[argh(option,short='b')]
    basetime_unix: i64,

    /// duration of each chat message
    #[argh(option, default="3")]
    duration: i32,
}

fn main() -> anyhow::Result<()> {
    let opts : Opts = argh::from_env();


    let mut num = 1;
    let mut subs = Vec::with_capacity(1000);

    for f in opts.files {
        let input = std::fs::read_to_string(f)?;
        let input : Comments  = miniserde::json::from_str(&input[..])?;
        for comment in input.comments {
            let d = chrono::DateTime::parse_from_rfc3339(&comment.updated_at[..])?;
            //println!("{} <{}> {}", d.timestamp() - opts.basetime_unix, comment.commenter.display_name, comment.message.body);
            let mut start_time = srtlib::Timestamp::new(0,0,0,0);
            start_time.add_seconds((d.timestamp() - opts.basetime_unix) as i32);
            let mut end_time = start_time.clone();
            end_time.add_seconds(opts.duration);
    
            let s = srtlib::Subtitle::new(
                num, 
                start_time,
                end_time, 
                format!("[{}] {}", comment.commenter.display_name, comment.message.body)
            );
            subs.push(s);
            num+=1;
        }
    }

    let subs = srtlib::Subtitles::new_from_vec(subs);
    println!("{}", subs.to_string());
    Ok(())
}
