use tokio::{
    fs::File,
    io::AsyncReadExt,
    sync::mpsc,
};
use std::error::Error;


fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

async fn get_lines_channel(mut file: File, tx: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> {
    let mut chunk = vec![0;8];
    let mut number_of_lines = 0;
    let mut curr_line_buffer = Vec::new();
    loop {
        let len = file.read(&mut chunk).await?;
        if len == 0 {
            // End of file
            break;
        }

        for &b in &chunk[..len] {
            if b == b'\n' {
                let curr_string = String::from_utf8_lossy(&curr_line_buffer).to_string();
                // println!("{:?}", curr_string);
                if let Err(_) = tx.send(curr_string).await {
                    println!("Receiver dropped!");
                    return Ok(());
                }
                number_of_lines += 1;
                curr_line_buffer.clear();
            } else {
                curr_line_buffer.push(b);
            }
        }
        // let curr_string = String::from_utf8_lossy(&chunk[..len]);
        // println!("{:?}", curr_string);
        // println!("Length of chunk is : {}", curr_string.len());
        // println!("Stack size of chunk in  : {}", std::mem::size_of_val(&curr_string));
        // println!("Stack pointer : {}", &curr_string.to_string());
        // println!("Heap pointer : {:?}", curr_string.as_ptr());
        // println!("                         ");
    }

    println!("The file has {} number of lines", number_of_lines);
    
    Ok(())
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("messages.txt").await?;
    // print_type(&file);

    let (tx, mut rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(100);

    // print_type(&rx);
    tokio::spawn(async move {
        get_lines_channel(file, tx).await;
    });

    while let Some(line) = rx.recv().await {
        println!("current line : {}", line);
    }
    
    Ok(())
}
