macro_rules! BZP_UPDATE_CRC { ($crcVar:expr, $cha:expr) => 
    {
        $crcVar = ($crcVar << 8) ^ ((*g_bzpCRC32Table.lock())[((($crcVar >> 24) as u8) ^ ($cha as u8))] as u32);
    }
}
pub(crate) use BZP_UPDATE_CRC;
