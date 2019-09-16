Mon September 16 11:41:10  2019 
cd into the root of the repo

rustup target add thumbv6m-none-eabi
cargo build

-> see runMe.sh shell script

Successfully compiles on TECRA laptop but not on LM White Tower
<code>   Compiling microbit v0.7.0                                                                                                       
error: Edition 2018 is unstable and only available for nightly builds of rustc.                                                    
                                                                                                                                   
error: Could not compile `microbit`.                                                                                               
warning: build failed, waiting for other jobs to finish...
error: build failed  
</code>

