pub trait FlashWriteErase<T: Copy> {
    // flash peripherals for write/erase access
    // these are seperate, as it is possible that on some platforms, write or erase might not require any access to the flash peripherals registers
    // in that case we don't want to limit that functionality by still requiring the peripheral to be passed for the write or erase operation
    type WritePeripheralAccess;
    type ErasePeripheralAccess;
    type WriteEraseError;

    /// erases all the pages needed for a write of Type T to the pointed to address
    unsafe fn page_erase(self, flash: &mut Self::WritePeripheralAccess) -> Result<(), Self::WriteEraseError>;
    unsafe fn write(self, flash: &mut Self::ErasePeripheralAccess, data: T) -> Result<(), Self::WriteEraseError>;
}

pub trait FlashRead<T: Copy> {
    type ReadAccess;
    type ReadError;

    unsafe fn read(self, flash: &mut Self::ReadAccess) -> Result<T, Self::ReadError>;
}