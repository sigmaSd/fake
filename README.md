# Fake
Create fake files with specified size

# Usage

***Create a file with the specified name and size, by default it will write random bytes to it***

`fake file_name file_size`

*Example:*

  `fake f 100MB`
  
  `fake f 1GB`

***Fill it with zeroes: `--zero`***

`fake file_name file_size --zero`

***Specify a maximum ram usage: `-b`***

`fake file_name file_size -b max_ram_usage`

*Example:*

  `fake f 1GB -b 50MB`
