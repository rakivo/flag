# `Remainder`
#### If you want to take all of the remained args, e.g. args is `-f 420 69 -r 555..1024 -m "urmom"`, and you call `parse_many`:
```
let flag = Flag::<String>::new("-r", "--range");
println!("{:?}", parser.parse_many(&flag, NArgs::Remainder));
```
### Output will be following: `555..1024 -m "urmom"`, simply all of the remained args to the right.
> <===================================================================================================>
# `SmartRemainder`
#### `SmartRemainder` works like `Remainder`, but it goes from left to right collects arguments until they're parsable, e.g. if args is `-f 420 69 555..1024 "rakivo"` and you call `parse_many`:
```
let flag = Flag::<i32>::new("-f", "--flag");
println!("{:?}", parser.parse_many(&flag, NArgs::SmartRemainder));
```
### Output will be following: `[420, 69]`.
> <===================================================================================================>
# `Count`
#### `Count` just calls `iter.take(count)`, i.e. takes `count` elements from left to right. e.g. args is `-f 420 69 1024 777 2048 -r 555..1024 -m "urmom"`:
```
let flag = Flag::<i32>::new("-f", "--flag");
println!("{:?}", parser.parse_many(&flag, NArgs::Count(3)));
```
### Output will be following: `[420, 69, 1024]`.
