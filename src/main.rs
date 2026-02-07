use tokio::{
    fs::File,
    io::AsyncReadExt,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()>{
    let mut file = File::open("messages.txt").await?;
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
                let curr_string = String::from_utf8_lossy(&curr_line_buffer);
                println!("{:?}", curr_string);
                number_of_lines += 1;
                curr_line_buffer = Vec::new();
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
