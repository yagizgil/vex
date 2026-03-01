Dynmamic Variables:
```
var score 100
score "a hundred"
```

Typical Variables:
```
var str name "vex"
var int age 1
var float pi 3.14
var bool active true
var list tags ["ai", "rust"]
var dict data {"key": "value"}
```

Includings, defines, macros
```
use "std/http"          # Include external modules
define MAX_RETRY 5      # Compile-time constants
macro #print to #prt    # Alias/Snippet replacement
```

Functions:
```
# fn [name] [param].[type] - [return]:
fn calculate_tax price.float rate.float - float:
    var res price * rate
    return res

# Usage
var total calculate_tax 500.0 0.2
```


Structs:
```
struct User:
    id.int
    name.str
    email.str
```

Implementations:
```
impl User:
    fn login:
        print f"User {self.name} is logging in..."

    fn get_id - int:
        return self.id
```

Conditionals:
```
if user.age > 18:
    print "Access Granted"
else:
    print "Access Denied"
```



Iteration:
```
for item in tags:
    print f"Processing {item}"

for i in range 0 10:
    print i
```



String Interpolation:
```
var str status "Online"
print f"System Status: {status}"
```



Pattern Matching:
```
var response fetch.json "url"

match response:
    ok val:
        print f"Success: {val}"
    err msg:
        print f"Error: {msg}"
```