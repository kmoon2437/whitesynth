pub struct SF2Info{}

pub struct SF2{
    pub info:SF2Info,
    pub raw:Vec<u8>
}

impl SF2{
    pub fn new(data:Vec<u8>) -> Self{
    let info = SF2Info{};
        return Self{
            info:info,
            raw:data
        };
    }
}