## QEMU device i2c-gpio.  

QEMU device demo based on QOM framework.  

____  
### 🌱 Soc  

```  
  -------------------------  
  | uart| i2c | spi | i2s | controller  
  -------------------------  
  | gpu | gpu | npu | tpu | processor  
  -------------------------  
  | cpu | cpu | cpu | cpu | processor  
  -------------------------  
```  

____  
### 🌱 Data Flow  

```  
   /-tx->- stop+8data+start ->-\ /-<- start+8data+stop -<-tx-\  
uart device                     |                       uart controller  
   \-rx-<- start+8data+stop -<-/ \->- stop+8data+start ->-rx-/  

   /-sda-<->- start+7addr(tx)+ack|nack(rx)+8data(tx)+ack|nack(rx)+stop -<->-sda-\  
i2c device                                                                  i2c controller  
   \-scl-<--- ------          ...12345678...123456789...         ----- ---<-scl-/  
```  

____  
### 🌱 Related File  

```  
./hw/i2c/Kconfig  
./hw/i2c/meson.build  

./include/hw/i2c/i2c_gpio.h  
./hw/i2c/i2c_gpio.c  
./include/hw/riscv/g233.h  
./hw/riscv/g233.c  

./rust/Cargo.toml  

./rust/hw/i2c/Kconfig  
./rust/hw/i2c/meson.build  

./rust/hw/i2c/i2c_gpio/meson.build  
./rust/hw/i2c/i2c_gpio/wrapper.h  
./rust/hw/i2c/i2c_gpio/Cargo.toml  
./rust/hw/i2c/i2c_gpio/build.rs  
```  

____  
### 📜 [MIT](LICENSE) License 许可证  

