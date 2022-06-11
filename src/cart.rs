pub struct Cart {
    pub prg_rom: Vec<u8>,
}

//TODO: remove me when we implement not terrible carts lmao
#[allow(dead_code)]
impl Cart {
    pub fn new(bank0: Vec<u8>) -> Self {
        return Cart { prg_rom: bank0 };
    }

    //TODO: these will vary WILDLY based upon the mapper right now we're just hardcoding it for nestest
    pub fn read(&mut self, addr: u16, length: usize) -> Vec<u8> {
        return self.prg_rom[(addr - 0xc000) as usize..(addr - 0xc000) as usize + length].into();
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        self.prg_rom[(addr - 0xc000) as usize] = byte;
    }
}
