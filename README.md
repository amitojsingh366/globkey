# globkey
A nodejs module for reading global keyevents, written in Rust

I'm not very good at nodejs or javascript in general, but I do know rust. So I used it to make a node module.


You use the library by defining a `.on` function or `.raw` function, this creates a callback that can be used to do whatever, here's an example

```javascript
let hook = require('globkey');

hook.on(function(action, key){
    if (action == "keydown" && key == 29) {
        console.log("Ctrl pressed down")
    } else if (action == "keyup" && key == 29) {
        console.log("Ctrl released")
    }
});
```

This example simply watches the Ctrl key and prints whether it was pressed down or released

The `.on` function returns 2 values, a string, and an integer. the string can either be **keyup** or **keydown** and the integer corresponds to the key pressed (I've included a script inside of the **keyfinder** directory for finding which keys corresponds with which numbers). 

There is also a `.raw` function, here is an example
```javascript
let hook = require('../electron_globkey/dist');

hook.raw(function(key){
    console.log(key)
});
```

This functon simply returns an array of each key that the user has pressed down every time the user releases or presses down a key

And that's it. I hope you find this library helpful. The module itself is contained in the **dist** folder
