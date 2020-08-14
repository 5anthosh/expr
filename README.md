# expr
> Simple scripting language in development

## Available functionality
```js

var variable = 234234;
variable = variable + 1;
print variable;
{
  var hello = "santhosh";
  print variable;
  var variable = 1234;
  {
    print variable;
    variable = 1222;
  }
  print variable;
  print hello;
}
print variable;

if (423423) {
  print true;
} else {
  print false;
}



fun printHello(a, b) {
while (a < 100000000000000000000000000000000) {
  print a;
  var temp = a;
  a = b;
  b = temp + b;
}
}

var start = clock();
printHello(0, 1);
print clock() - start + " sec";
print clock;
print clock;

print clock();

fun fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 2) + fibonacci(n - 1);
}

for (var i = 0; i < 20; i = i + 1) {
  print fibonacci(i);
}

fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var fib = fibonacci;
print fib(10);


fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter();
counter();

var i = 6;
fun first() {
    var i = 0;
    fun second() {
        print "Does it have reference " + i ;
        i = i + 1;
        var j = i;
        fun third() {
            i = 5;
            print (i + " " + j);
        }
        return third;
    }
    return second;
}

var second = first();
var third = second();
third();
second();
third();

print random_alphanumeric();
for(var i = 1.1 ; i < 101; i = i + 1){
print random() * 1000;
}
```
