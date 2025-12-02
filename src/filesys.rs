#[derive(Debug)]
pub enum FileErr{
    StdFileErr(std::io::Error),
    InvalidPermsFormat
}
impl From<std::io::Error> for FileErr {
    fn from(err: std::io::Error) -> Self {
        FileErr::StdFileErr(err)
    }
}
impl std::fmt::Display for FileErr {
    fn fmt(self: &Self, f:  &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            FileErr::StdFileErr(e) => write!(f, "std IO err :{} !", e),
            FileErr::InvalidPermsFormat => write!(f, "Invalid format. Expects only octals of length: 3/4!"),
        }
    }
}

pub(crate) fn chmod(perms: &str, path: &str) -> Result<(), FileErr>{
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    if perms.trim().len() != 3 && perms.trim().len() != 4{
        return Err(FileErr::InvalidPermsFormat);        
    } 
    let testFormat =  perms.trim().parse::<u32>();
    match testFormat{
        Err(e) => return Err(FileErr::InvalidPermsFormat),
        _ => (),
    }
    for digit in perms.chars() {
        if !('0'..='7').contains(&digit) {  //equivalent digit.is_ascii_octdigit() == false 
            return Err(FileErr::InvalidPermsFormat);
        }
    }
    let metadata = fs::metadata(path)?;
    let mut permissions = metadata.permissions();
    let perms_num = u32::from_str_radix(perms.trim(), 8)
                        .map_err(|_| FileErr::InvalidPermsFormat)?;
    //permissions.set_mode(permissions.mode() | testFormat.unwrap()); //with bitwise OR
    permissions.set_mode(perms_num);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

pub(crate) fn cp(args: &[String]) -> Result<(), FileErr>{
    use std::fs;
    let mut rec: bool = false;
    let mut startidx = 0;
    if args.len() < 2{
        return Err(FileErr::StdFileErr(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not enough arguments for cp!")));
    }
    if args[0].cmp(&"-r".to_string()) == std::cmp::Ordering::Equal {
        rec = true;
        startidx += 1;
    }
    let source = &args[startidx];
    let destination = &args[startidx + 1];
    assert!(fs::exists(source).expect("Can't check existence of file source"));
    //assert!(fs::exists(destination).expect("Can't check existence of file destination"));
    let metadataSource = fs::metadata(source).unwrap();
    //let metadataDest = fs::metadata(destination).unwrap();
    if !rec{
        if metadataSource.is_dir() { //|| metadataDest.is_dir(){
            return Err(FileErr:: StdFileErr(std::io::Error::new(std::io::ErrorKind::InvalidFilename, "Invalid entries!")));
        }
        fs::copy(source, destination)?;
    } else {
        if !metadataSource.is_dir() { //|| !metadataDest.is_dir(){
            return Err(FileErr:: StdFileErr(std::io::Error::new(std::io::ErrorKind::InvalidFilename, "Invalid entries!")));
            //TODO; complete implementaton for recursive case
        }
    }
    Ok(())
}