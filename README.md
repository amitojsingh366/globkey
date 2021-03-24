# globkey
A nodejs module for reading global keyevents, written in Rust

I'm not very good at nodejs or javascript in general, but I do know rust. So I used it to make a node module.


You use the library by defining a `.raw` function, this creates a callback that can be used to do whatever, here's an example
```javascript
let hook = require('../electron_globkey/dist');

hook.raw(function(key){
    console.log(key)
});
```

This functon simply returns an array of each key that the user has pressed down every time the user releases or presses down a key

And that's it. I hope you find this library helpful. The module itself is contained in the **dist** folder
