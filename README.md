# RustyBox
## Rust@UPB Assignment1

RustyBox is a program that provides a command-line interface encompassing essential commands, all bundled into a single binary. While graphical interfaces may appear more user-friendly, command-line interfaces are often 	<ins> simpler and more efficient </ins>. Because of their simplicity, command-line tools consume fewer resources, making them ideal for environments with limited memory or processing power, such as embedded systems (routers, medical devices, etc.) or extremely lightweight Docker containers (like Alpine Linux).
Once users become familiar with the command-line interface, it enables more precise and powerful control over the system.
Why choose RustyBox over traditional _core utilities_? By combining multiple commands into one **statically linked binary**, RustyBox minimizes storage requirements and memory usage. This makes it particularly suitable for routers, industrial systems, and other resource-constrained devices

| Command | Utility & Flags |
|-----------------|----------------|
| grep  | standard Unix  |
| cp   | supports Recursive flag |
| chmod  | implements StickyBit   |
| cat  | implements StickyBit   |
| echo  | implements StickyBit   |
