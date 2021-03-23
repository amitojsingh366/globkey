let hook = require('../dist');

hook.on(function(action, key){
    if (action == "keydown") {
        console.log(key)
    }
});