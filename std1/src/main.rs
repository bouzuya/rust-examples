fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::{
        io::{BufWriter, Write},
        time::Instant,
    };

    #[test]
    fn test_lock_and_buffered() -> anyhow::Result<()> {
        let stdout = std::io::stdout();
        let stdout = stdout.lock();
        let mut writer = BufWriter::new(stdout);
        let instant = Instant::now();
        for i in 0..1024 * 1024 {
            writer.write_all(i.to_string().as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;

        // ~6.69s
        let duration = instant.elapsed();
        println!("{:?}", duration);
        Ok(())
    }

    #[test]
    fn test_lock() -> anyhow::Result<()> {
        let stdout = std::io::stdout();
        let mut writer = stdout.lock();
        let instant = Instant::now();
        for i in 0..1024 * 1024 {
            writer.write_all(i.to_string().as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;

        // ~35.95s
        let duration = instant.elapsed();
        println!("{:?}", duration);
        Ok(())
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut writer = std::io::stdout();
        let instant = Instant::now();
        for i in 0..1024 * 1024 {
            writer.write_all(i.to_string().as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;

        // ~37.77s
        let duration = instant.elapsed();
        println!("{:?}", duration);
        Ok(())
    }

    #[test]
    fn test_no_newline() -> anyhow::Result<()> {
        let mut writer = std::io::stdout();
        let instant = Instant::now();
        for i in 0..1024 * 1024 {
            writer.write_all(i.to_string().as_bytes())?;
            // remove \n
            writer.write_all(b"a")?;
        }
        writer.flush()?;

        // ~4.85s
        let duration = instant.elapsed();
        println!("{:?}", duration);
        Ok(())
    }
}
