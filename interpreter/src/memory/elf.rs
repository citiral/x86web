use super::*;
use goblin::elf::Elf;
use goblin::elf::header::*;
use goblin::elf::program_header::ProgramHeader;
use goblin::elf::section_header::SectionHeader;
use crate::*;

fn symbol {}

impl Memory {
    fn get_u32(&self, bytes: &[u8], offset: u32) -> u32
    {
        let i = offset as usize;
        (bytes[i] as u32 >> 24) + (bytes[i + 1] as u32 >> 16) + (bytes[i + 2] as u32 >> 8) + (bytes[i + 3] as u32 >> 24)
    }

    fn set_u32(&mut self, bytes: &[u8], offset: u32, value: u32)
    {   
        self.set_at(offset    , ((value & 0x000000FF)      ) as u8);
        self.set_at(offset + 1, ((value & 0x0000FF00) >> 8 ) as u8);
        self.set_at(offset + 2, ((value & 0x00FF0000) >> 16) as u8);
        self.set_at(offset + 3, ((value & 0xFF000000) >> 24) as u8);
    }

    fn load_program_header(&mut self, bytes: &[u8], ph: &ProgramHeader)
    {
        let offset = ph.p_offset as u32;
        let vaddr = ph.p_vaddr as u32;
        let copysize = ph.p_filesz as u32;
        let zerosize = ph.p_memsz  as u32 - copysize;

        for i in 0..copysize {
            self.set_at(vaddr + i, bytes[(offset + i) as usize]);
        }

        for i in 0..zerosize {
            self.set_at(vaddr + copysize + i, 0);
        }
    }

    fn handle_relocation_header(&mut self, bytes: &[u8], sh: &SectionHeader)
    {
        let size = sh.sh_size as u32;
        let offset = sh.sh_offset as u32;

        let relocation_count = size / 0x8;

        for i in 0..relocation_count {
            let r_offset = self.get_u32(bytes, offset + i * 8);
            let r_into = self.get_u32(bytes, offset + i * 8 + 4);


        }
    }

    pub fn link_elf(&mut self, bytes: &[u8], elf: &Elf) -> Result<u32, &str> {
        if elf.header.e_machine != EM_386 {
            return Err("Invalid machine type.");
        }

        // load every program header
        elf.program_headers.iter()
                           .filter(|ph| ph.p_type == goblin::elf::program_header::PT_LOAD)
                           .for_each(|ph| self.load_program_header(bytes, ph));



        // parse symbol tables
        elf.section_headers.iter()
                           .filter(|sh| sh.sh_type == goblin::elf::section_header::SH_DYNSYM)
                           .for_each(|sh| self.handle_relocation_header(bytes, sh));

        // relocate where needed
        elf.section_headers.iter()
                           .filter(|sh| sh.is_relocation())
                           .for_each(|sh| self.handle_relocation_header(bytes, sh));

        // check if the header is valid
        return Ok(elf.header.e_entry as u32);
    }
}