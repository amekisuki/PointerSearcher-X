use std::{fs::File, os::unix::fs::FileExt, path::Path};

use super::{
    dump::{create_pointer_map, create_pointer_map_file},
    info::{list_image_maps, list_image_maps_pince, list_unknown_maps},
    Error, PointerMap, RangeMap, RangeSet,
};

pub struct Process {
    pid: i32,
    mem: File,
}

impl Process {
    pub fn attach(pid: i32) -> Result<Self, Error> {
        let mem =
            File::open(format!("/proc/{pid}/mem")).map_err(|err| Error::AttachProcess(err))?;
        Ok(Self { pid, mem })
    }

    pub fn list_image_maps(&self) -> Result<RangeMap<usize, String>, Error> {
        list_image_maps(self.pid).map_err(|err| Error::QueryProcess(err))
    }

    pub fn list_image_maps_pince(&self) -> Result<RangeMap<usize, String>, Error> {
        list_image_maps_pince(self.pid).map_err(|err| Error::QueryProcess(err))
    }

    pub fn list_unknown_maps(&self) -> Result<RangeSet<usize>, Error> {
        list_unknown_maps(self.pid).map_err(|err| Error::QueryProcess(err))
    }

    pub fn create_pointer_map_file(
        &self,
        module_maps: RangeMap<usize, String>,
        unknown_maps: RangeSet<usize>,
        path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        create_pointer_map_file(&self.mem, module_maps, unknown_maps, path)
    }

    pub fn create_pointer_map(
        &self,
        module_maps: RangeMap<usize, String>,
        unknown_maps: RangeSet<usize>,
    ) -> Result<PointerMap, Error> {
        create_pointer_map(&self.mem, module_maps, unknown_maps)
    }

    pub fn read_memory_exact(&self, addr: usize, buf: &mut [u8]) -> Result<(), Error> {
        self.mem
            .read_exact_at(buf, addr as u64)
            .map_err(|err| Error::ReadMemory(err))
    }
}
