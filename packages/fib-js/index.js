exports.fib= function fib(n) {
  if (n === 1 || n === 2) {
    return 1
  }
  return fib(n - 1) + fib(n - 2)
}

// function fib2(n) { 
//     if (n < 2) { 
//         return 1; 
//     } 
//     var a = 1, b = 1; 
//     for (var i = 2; i < n - 1 ;i++ ) { 
//         b = a + b; 
//         a = b - a; 
//     } 
//     return a + b; 
// }