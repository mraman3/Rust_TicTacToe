<!-- ABOUT THE PROJECT -->
## About The Project
This is my first-ever attempt at programming with rust and I have decided to pursue a simple tic-tac-toe project. 

The UI is terminal this was done so purposely as I intended to implement Iroh a p2p networking in an attempt to make the game online. With maybe 
later adding a fun and interactive GUI. My first goal is to learn the basics of Rust and I have done so with adequate Unit tests, functions, documentation, and other aspects of Rust that I am new to. 

### Built With
<p align="center">
  <a href="https://www.rust-lang.org/">
    <img width="100" height="100" src="https://github.com/tandpfun/skill-icons/blob/main/icons/Rust.svg" />
  </a>
  <a href="https://www.iroh.computer/">
    <img width="290" height="110" src="https://www.iroh.computer/img/logo/iroh-wordmark-purple.svg" />
  </a>
</p>

Build
-----
    $ cargo build

Run
-----
The host side will run, which will produce the command for the client to run after the endpoint is created

    $ cargo run -- host 
    
This is an example of what the client-side command will look like 

```
$ cargo run -- client --node-id 2f0cd88cfa1864242c728c70998092fb6169afd455e1bace1225632658612f09 --addrs "24.150.21.28:59005 192.168.40.5:59005 [2001:1970:564d:ec00:81e:1daa:2c1a:bfd2]:59006 [2001:1970:564d:ec00:6457:77de:ca60:e435]:59006 [2001:1970:564d:ec00:f1be:2e7c:65cb:fab3]:59006"
```


Example Output 
-----
![output](https://github.com/user-attachments/assets/865b2c5d-ad8d-4162-90fe-0471e399ec72)





