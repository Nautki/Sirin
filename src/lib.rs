use std::future::IntoFuture;

use embedded_hal_async::spi::ErrorKind;
use spi_handle::SpiHandle;
use embedded_hal_async::spi::SpiBus;

pub struct W25Q<S: SpiHandle>{
    spi: S
}

pub struct Log{
    current_index : usize
}

impl <S: SpiHandle> W25Q<S>{

    //write enable
    pub async fn write_enable(&mut self) -> Result<(), ErrorKind> {
        let mut spi = self.spi.select().await;
        spi.write(&[0x06]).await?;
        Ok(())
    }

    //page program 02h
    pub async fn page(&mut self, addr: u32, words: &[u8]) -> Result<(), ErrorKind> {
        let mut spi = self.spi.select().await;
        spi.write(&[0x02]).await?;
        spi.write(&[((addr >> 16) & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8, ((addr & 0xFF) as u8)]).await?;
        spi.write(words).await?;
        Ok(())
    }

    //sector erase 20h
    pub async fn sector_erase(&mut self, addr: u32) -> Result<(), ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0x20]).await?;
        spi.write(&[((addr >> 16) & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8, ((addr & 0xFF) as u8)]).await?;
        Ok(())
    }

    //read data 03h
    pub async fn read_data(&mut self, addr: u32, words: &mut[u8]) -> Result<(), ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0x03]).await?;
        spi.write(&[((addr >> 16) & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8, ((addr & 0xFF) as u8)]).await?;
        spi.read(words).await?;
        Ok(())
    }

    //all non-fast read, writes, erases should be implemented
    //chip erase
    pub async fn chip_erase(&mut self) -> Result<(), ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0x60]).await?;
        Ok(())
    }

    //block erase 64KB D8h
    pub async fn block_64kb_erase(&mut self, addr:u32) -> Result<(), ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0xD8]).await?;
        spi.write(&[((addr >> 16) & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8, ((addr & 0xFF) as u8)]).await?;
        Ok(())
    }

    //block erase 32KB 52h
    pub async fn block_32kb_erase(&mut self, addr:u32) -> Result<(), ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0x52]).await?;
        spi.write(&[((addr >> 16) & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8, ((addr & 0xFF) as u8)]).await?;
        Ok(())
    }

    //erase/program suspend 75h
    pub async fn suspend(&mut self) -> Result<(), ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0x75]).await?;
        Ok(())
    }

    //erase/program resume 7Ah
    pub async fn resume(&mut self) -> Result<(), ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0x7A]).await?;
        Ok(())
    }


    //read device_id 90h
    pub async fn read_device_id(&mut self) -> Result<u8, ErrorKind>{
        let mut spi = self.spi.select().await;
        spi.write(&[0x90]).await?;
        spi.write(&[0,0,0]).await?;
        let mut array: [u8; 2] = [0; 2];
        spi.read(&mut array).await?;  
        Ok(array[1])
    }

    //read manufacturer id --returns a constant EFh
    pub fn read_manufacturer_id(&mut self) -> u16{
        0xEF
    }

    //write to block with a block pointer(point to last erase / the current block writing to)
    //wrap around when reaches the end.
    pub async fn log(&mut self, arr: &mut[u8], words: &[u8] ,current_index : *mut u8) -> Result<(),ErrorKind>{
        let mut spi = self.spi.select().await;
        unsafe { current_index.add(words.len())};
        let last_ptr = unsafe { arr.as_mut_ptr().add(arr.len()) };

        if current_index > last_ptr{
            current_index = arr.as_mut_ptr() //we set the current to the beginning = 0
        }
        self.sector_erase(*current_index); //sector_erase takes u32, do we erase 4 sets of the index??
        Ok(())
    }
    
    
    }

    