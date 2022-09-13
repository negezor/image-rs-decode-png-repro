use std::io::Read;

fn main() {
    let path_to_file = "./broken.png";

    let mut f = std::fs::File::open(path_to_file).expect("no file found");
    let metadata = std::fs::metadata(&path_to_file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let format = image::guess_format(&buffer).unwrap();

    let mut reader = image::io::Reader::with_format(std::io::Cursor::new(buffer), format);

    reader.no_limits();

    let im = reader.decode().unwrap();
}
