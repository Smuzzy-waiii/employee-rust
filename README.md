# employee-rust
smol employee-dept add system written in rust using the module system extensively.

## Usage
Programs is a REPL

```
employee > sdks
Invalid command entered!
employee > add Smuz to Engineering
employee > add PIL to Operations    
employee > display
Employee { name: "Smuz", dept: Engineering }
Employee { name: "PIL", dept: Operations }
employee > exit
```

**TODO**: 
- Shift main logic to lib.rs to make it idiomatic and be able to add integration tests. 
- Add tests, both unit test and integration tests
