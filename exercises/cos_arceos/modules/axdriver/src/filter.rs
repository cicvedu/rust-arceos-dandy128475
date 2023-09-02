/*pub struct NetFilter<T> {
    pub inner: T,
}*/
//use axdriver::{DevResult, RawVirtioPtr};

/*use driver_common::{BaseDriverOps, DeviceType, DevResult};
use log::warn;

pub struct NetFilter<T> {
    pub inner: T,
}

impl<T> NetFilter<T> {
    pub fn new(inner: T) -> Self {
        NetFilter { inner }
    }
    
}*/
//use crate::EthernetAddress;
use crate::BaseDriverOps;
use crate::DeviceType;
//use log::warn;
use driver_net::{NetDriverOps, DevResult, NetBufPtr, DevError};

pub struct NetFilter<T> {
   pub inner: T,
}

impl<T> NetFilter<T> {
    pub fn new(inner: T) -> Self {
        NetFilter { inner }
    }

    pub fn get_inner(&self) -> &T {
        &self.inner
    }
}

impl<T: BaseDriverOps> BaseDriverOps for NetFilter<T> {
    fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }
}

impl<T: NetDriverOps> NetDriverOps for NetFilter<T> {
     
    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        // 在 transmit 方法前加入过滤逻辑
        // ...
        warn!("Filter: transmit len[{}]", tx_buf.packet_len());

        // 调用内部实现的 transmit 方法
        self.inner.transmit(tx_buf)
    }

    fn receive(&mut self) -> Result<NetBufPtr, DevError> {
        // 在 receive 方法前加入过滤逻辑
        // ...

        // 调用内部实现的 receive 方法
        self.inner.receive()
    }

    // 其他 NetDriverOps 的方法也可以在这里实现

    
    fn mac_address(&self) -> driver_net::EthernetAddress {
        // 假设 NetDriverOps 中有一个方法可以获取 MAC 地址
        // 请根据实际情况调用适当的方法来获取 MAC 地址
        // 这里仅作示例，假设存在一个名为 get_mac_address 的方法
        self.inner.mac_address()
    }

    fn can_transmit(&self) -> bool {
        // 实现检查是否可以发送数据的逻辑
        // 如果不需要特定的实现，可以使用默认的 todo!() 占位符
        //todo!()
        
            // 示例：检查发送队列是否有空闲空间
            // 这是一种常见的条件之一，检查是否可以继续发送数据
            self.inner.can_transmit()
            
    }

    fn can_receive(&self) -> bool {
        // 实现检查是否可以接收数据的逻辑
        // 如果不需要特定的实现，可以使用默认的 todo!() 占位符
        self.inner.can_receive()
    }

    fn rx_queue_size(&self) -> usize {
        // 实现获取接收队列大小的逻辑
        // 如果不需要特定的实现，可以使用默认的 todo!() 占位符
        self.inner.rx_queue_size()
    }

    fn tx_queue_size(&self) -> usize {
        // 实现获取发送队列大小的逻辑
        // 如果不需要特定的实现，可以使用默认的 todo!() 占位符
        self.inner.tx_queue_size()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        // 实现回收接收缓冲区的逻辑
        // 如果不需要特定的实现，可以使用默认的 todo!() 占位符
        warn!("Filter: receive len[{:?}]", rx_buf.packet_len());

        self.inner.recycle_rx_buffer(rx_buf)
    }

    /*fn recycle_tx_buffers(&mut self) -> Result<(), DevError> {
        // 实现回收发送缓冲区的逻辑
        // 如果不需要特定的实现，可以使用默认的 todo!() 占位符
        todo!()
    }*/
    fn recycle_tx_buffers(&mut self) -> DevResult {
        // 实现回收发送缓冲区的逻辑
        // 如果不需要特定的实现，可以使用默认的 `Ok(())`
        // 如果出现错误，可以返回 `Err` 并提供适当的错误信息
        // 例如：`Err(DevError::Custom("Failed to recycle tx buffers".to_string()))`
        // 如果没有特定的实现，也可以使用默认的 `todo!()`
        Ok(())
    }
    

   /*  fn alloc_tx_buffer(&mut self, len: usize) -> Result<NetBufPtr, DevError> {
        // 假设我们有一个名为 `allocate_buffer` 的函数来分配发送缓冲区
        match allocate_buffer(len) {
            Some(buffer) => Ok(buffer),
            None => Err(DevError::NoMemory),
        }
    
    }   */
    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        self.inner.alloc_tx_buffer(size)
    }

    
       
}











